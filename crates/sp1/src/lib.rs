use sp1_sdk::{include_elf, EnvProver, SP1ProvingKey, SP1VerifyingKey};

pub const SP1_SIGSTORE_ELF: &[u8] = include_elf!("sigstore-sp1-program");

pub fn vk(elf: &[u8]) -> SP1VerifyingKey {
    let env_prover = EnvProver::new();
    let (_, vk) = env_prover.setup(elf);
    vk
}

pub fn pk(elf: &[u8]) -> SP1ProvingKey {
    let env_prover = EnvProver::new();
    let (pk, _) = env_prover.setup(elf);
    pk
}