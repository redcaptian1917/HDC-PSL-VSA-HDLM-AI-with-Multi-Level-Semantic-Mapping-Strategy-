// ============================================================
// LFI Genetic Optimization Layer — Evolutionary Metabolism
// Section 1.IV: "Uses GA to continuously mutate security thresholds
// based on Pixel battery thermal limits and threat telemetry."
// ============================================================

use rand::Rng;

/// A chromosome representing a set of hyper-parameters
/// (e.g., PSL thresholds, LNN time constants).
#[derive(Clone)]
pub struct Chromosome {
    pub genes: Vec<f64>,
    pub fitness: f64,
}

/// The Genetic Optimizer for biological-style parameter tuning.
pub struct GeneticOptimizer {
    population: Vec<Chromosome>,
    mutation_rate: f64,
}

impl GeneticOptimizer {
    /// Initialize a population of N chromosomes with G genes each.
    pub fn new(pop_size: usize, gene_count: usize) -> Self {
        debuglog!("GeneticOptimizer::new: pop={}, genes={}", pop_size, gene_count);
        let mut rng = rand::thread_rng();
        let mut population = Vec::with_capacity(pop_size);
        for _ in 0..pop_size {
            let genes: Vec<f64> = (0..gene_count).map(|_| rng.gen_range(0.0..1.0)).collect();
            population.push(Chromosome { genes, fitness: 0.0 });
        }
        Self { population, mutation_rate: 0.05 }
    }

    /// Mutate and crossover the population to produce the next generation.
    pub fn evolve(&mut self) {
        debuglog!("GeneticOptimizer::evolve: Generating next generation...");
        let mut rng = rand::thread_rng();
        let pop_size = self.population.len();
        
        // Sort by fitness (simulated)
        self.population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap_or(std::cmp::Ordering::Equal));
        
        // Elite keep: Top 20%
        let elite_count = pop_size / 5;
        let mut next_gen = self.population[0..elite_count].to_vec();
        
        // Fill remaining with crossover and mutation
        while next_gen.len() < pop_size {
            let parent_a = &self.population[rng.gen_range(0..elite_count)];
            let parent_b = &self.population[rng.gen_range(0..elite_count)];
            
            // Crossover
            let mut child_genes = Vec::with_capacity(parent_a.genes.len());
            for i in 0..parent_a.genes.len() {
                let gene = if rng.gen_bool(0.5) { parent_a.genes[i] } else { parent_b.genes[i] };
                
                // Mutation
                let mutated_gene = if rng.gen_bool(self.mutation_rate) {
                    gene * rng.gen_range(0.8..1.2)
                } else {
                    gene
                };
                child_genes.push(mutated_gene.clamp(0.0, 1.0));
            }
            
            next_gen.push(Chromosome { genes: child_genes, fitness: 0.0 });
        }
        
        self.population = next_gen;
    }

    /// Update the fitness of a specific chromosome.
    pub fn update_fitness(&mut self, idx: usize, fitness: f64) {
        if let Some(c) = self.population.get_mut(idx) {
            c.fitness = fitness;
        }
    }

    /// Retrieve the best parameters found so far.
    pub fn best_genes(&self) -> Option<&Vec<f64>> {
        self.population.first().map(|c| &c.genes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evolution_cycle() {
        let mut optimizer = GeneticOptimizer::new(10, 5);
        for i in 0..10 {
            optimizer.update_fitness(i, i as f64);
        }
        optimizer.evolve();
        assert_eq!(optimizer.population.len(), 10);
        assert!(optimizer.population[0].genes.len() == 5);
    }

    #[test]
    fn test_population_initialization() {
        let opt = GeneticOptimizer::new(20, 8);
        assert_eq!(opt.population.len(), 20);
        for c in &opt.population {
            assert_eq!(c.genes.len(), 8);
            assert_eq!(c.fitness, 0.0);
            for &g in &c.genes {
                assert!(g >= 0.0 && g <= 1.0, "Genes should be in [0,1]");
            }
        }
    }

    #[test]
    fn test_fitness_update() {
        let mut opt = GeneticOptimizer::new(5, 3);
        opt.update_fitness(0, 0.95);
        assert!((opt.population[0].fitness - 0.95).abs() < 0.001);

        // Out of bounds index should not panic.
        opt.update_fitness(999, 1.0);
    }

    #[test]
    fn test_best_genes() {
        let mut opt = GeneticOptimizer::new(5, 3);
        for i in 0..5 {
            opt.update_fitness(i, (i + 1) as f64);
        }
        opt.evolve(); // Sort by fitness, highest first
        let best = opt.best_genes().expect("should have best");
        assert_eq!(best.len(), 3);
    }

    #[test]
    fn test_elite_preservation() {
        let mut opt = GeneticOptimizer::new(10, 4);
        // Give the first chromosome maximum fitness.
        opt.update_fitness(0, 100.0);
        let elite_genes = opt.population[0].genes.clone();
        opt.evolve();
        // After evolution, the elite should be preserved (first in sorted order).
        assert_eq!(opt.population[0].genes, elite_genes,
            "Best chromosome should survive evolution");
    }

    #[test]
    fn test_multiple_evolution_cycles() {
        let mut opt = GeneticOptimizer::new(20, 6);
        for gen in 0..10 {
            for i in 0..20 {
                opt.update_fitness(i, (i as f64) * (gen as f64 + 1.0));
            }
            opt.evolve();
        }
        // Population should still be valid after 10 generations.
        assert_eq!(opt.population.len(), 20);
        for c in &opt.population {
            assert_eq!(c.genes.len(), 6);
            for &g in &c.genes {
                assert!(g >= 0.0 && g <= 1.0, "Gene mutation should stay in bounds");
            }
        }
    }

    #[test]
    fn test_small_population() {
        // Minimum viable population: 5 (elite = 1).
        let mut opt = GeneticOptimizer::new(5, 2);
        opt.update_fitness(0, 1.0);
        opt.evolve();
        assert_eq!(opt.population.len(), 5);
    }
}
