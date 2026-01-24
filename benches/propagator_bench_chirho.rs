// For God so loved the world that he gave his only begotten Son,
//     that whoever believes in him should not perish but have eternal life.
//     John 3:16

//! Benchmarks for propagator operations.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use propagators_chirho::{
    CellChirho, ConstraintSystemChirho, IntervalAdderChirho, IntervalChirho, NumericInfoChirho,
    SchedulerChirho,
};

fn benchmark_interval_operations_chirho(c_chirho: &mut Criterion) {
    let a_chirho = IntervalChirho::new_chirho(1.0, 10.0);
    let b_chirho = IntervalChirho::new_chirho(2.0, 8.0);

    c_chirho.bench_function("interval_intersect", |bench_chirho| {
        bench_chirho.iter(|| black_box(a_chirho.intersect_chirho(&b_chirho)))
    });

    c_chirho.bench_function("interval_add", |bench_chirho| {
        bench_chirho.iter(|| black_box(a_chirho.add_chirho(&b_chirho)))
    });

    c_chirho.bench_function("interval_mul", |bench_chirho| {
        bench_chirho.iter(|| black_box(a_chirho.mul_chirho(&b_chirho)))
    });

    c_chirho.bench_function("interval_square", |bench_chirho| {
        bench_chirho.iter(|| black_box(a_chirho.square_chirho()))
    });
}

fn benchmark_merge_operations_chirho(c_chirho: &mut Criterion) {
    let a_chirho = NumericInfoChirho::interval_chirho(0.0, 100.0);
    let b_chirho = NumericInfoChirho::interval_chirho(50.0, 150.0);

    c_chirho.bench_function("numeric_info_merge", |bench_chirho| {
        bench_chirho.iter(|| black_box(a_chirho.merge_chirho(&b_chirho)))
    });
}

fn benchmark_propagation_chirho(c_chirho: &mut Criterion) {
    c_chirho.bench_function("simple_addition_network", |bench_chirho| {
        bench_chirho.iter(|| {
            let scheduler_chirho = SchedulerChirho::new_chirho();
            let a_chirho = CellChirho::new_chirho("a");
            let b_chirho = CellChirho::new_chirho("b");
            let c_chirho = CellChirho::new_chirho("c");

            IntervalAdderChirho::install_chirho(
                a_chirho.clone(),
                b_chirho.clone(),
                c_chirho.clone(),
                &scheduler_chirho,
            );

            a_chirho.add_content_chirho(NumericInfoChirho::exact_chirho(3.0), &scheduler_chirho);
            b_chirho.add_content_chirho(NumericInfoChirho::exact_chirho(4.0), &scheduler_chirho);

            scheduler_chirho.run_chirho();

            black_box(c_chirho.content_chirho())
        })
    });
}

fn benchmark_constraint_system_chirho(c_chirho: &mut Criterion) {
    c_chirho.bench_function("temperature_conversion", |bench_chirho| {
        bench_chirho.iter(|| {
            let mut system_chirho = ConstraintSystemChirho::new_chirho();

            let celsius_chirho = system_chirho.make_cell_chirho("celsius");
            let fahrenheit_chirho = system_chirho.make_cell_chirho("fahrenheit");
            let nine_fifths_chirho = system_chirho.make_cell_chirho("nine_fifths");
            let thirty_two_chirho = system_chirho.make_cell_chirho("thirty_two");
            let product_chirho = system_chirho.make_cell_chirho("product");

            system_chirho.set_exact_chirho(&nine_fifths_chirho, 1.8);
            system_chirho.set_exact_chirho(&thirty_two_chirho, 32.0);

            system_chirho.add_multiplier_chirho(
                &celsius_chirho,
                &nine_fifths_chirho,
                &product_chirho,
            );
            system_chirho.add_adder_chirho(&product_chirho, &thirty_two_chirho, &fahrenheit_chirho);

            system_chirho.set_exact_chirho(&celsius_chirho, 100.0);
            system_chirho.run_chirho();

            black_box(system_chirho.get_chirho(&fahrenheit_chirho))
        })
    });
}

criterion_group!(
    benches_chirho,
    benchmark_interval_operations_chirho,
    benchmark_merge_operations_chirho,
    benchmark_propagation_chirho,
    benchmark_constraint_system_chirho,
);

criterion_main!(benches_chirho);
