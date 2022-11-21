use std::time::Instant;

// This method represents the Sieve of Eratosthenes.
fn main() {
    let then = Instant::now();

    // A bit for each "is_prime" up to N.
    const N: usize = u32::MAX as usize;
    const PADDING: usize = N % 8; // Need extra padding when not exactly divisble by 8.
    let mut is_prime = vec![0b11111111 as u8; N / 8 + (PADDING > 0) as usize];

    let mut count = N - 1; // Account for 1 not being prime.

    for p in 2..=N as usize {
        // The bit index is one back from the one that would be calculated.
        // This is due to a lack of "0" representation when marking primes.
        let i = p - 1;

        // If the bit has already been set false, this has already been accounted for.
        if is_prime[i / 8] & (1u8 << (i % 8)) == 0 {
            count -= 1; // Every time we detect a false marking, decrement the counter.
            continue;
        }

        let mut q = p * 2;

        while q <= N {
            let i = q - 1; // Shift back one bit as per the above comment.
            is_prime[i / 8] &= !(1u8 << (i % 8));
            q += p;
        }
    }

    // The number of primes from the end of the array to show on screen.
    const DEMO_LENGTH: usize = 50;
    let mut demo_count = 0;
    let mut p = N; // Go back from the end.

    println!("\nLast {DEMO_LENGTH} primes found: ");
    while demo_count < DEMO_LENGTH {
        let i = p - 1; // Index correction, as per before.
        if is_prime[i / 8] & (1u8 << (i % 8)) != 0 {
            print!("{}\t", p);
            demo_count += 1;
            if demo_count % 5 == 0 {
                // Split the output into rows.
                print!("\n");
            }
        }
        p -= 1;
    }

    println!(
        "\n{} primes found in the range 1..{} in {:.2?}\n",
        count,
        N,
        then.elapsed()
    );
}
