use std::net::Ipv4Addr;

pub struct V4 {
    pub octets: Vec<u8>
}

pub struct V6 {
    pub octets: Vec<u8>
}

impl V4 {
    pub fn new(octets: Vec<u8>) -> Self {
        Self { octets }
    }

    pub fn to_decimal(&self) -> u32 {
        self.octets
            .iter()
            .rev()
            .enumerate()
            .map(|(index, &octet)| { (octet as u32) << (index * 8) })
            .sum()
    }

    pub fn to_octal(&self) -> String {
        self.octets
            .iter()
            .map(|octet| octet_to_octal(octet))
            .collect::<Vec<_>>()
            .join(".")
    }

    pub fn to_hex(&self) -> String {
        self.octets
            .iter()
            .map(|octet| octet_to_hex(octet))
            .collect::<Vec<_>>()
            .join(".")
    }

    pub fn permutations(&self) -> Vec<String> {
        let max_foldings = self.octets.len();
        let mut possibilities = Vec::new();

        for foldings in 1..max_foldings {
            let octet_suffix: Vec<&u8> = self.octets.iter().rev().take(foldings).rev().collect();
            let decimal_value = to_decimal(&octet_suffix).to_string();

            let mut folding_permutations = Vec::new();

            for octet in self.octets.iter().rev().skip(foldings).rev() {
                if folding_permutations.is_empty() {
                    folding_permutations.push(octet.to_string());
                    folding_permutations.push(octet_to_octal(octet));
                    folding_permutations.push(octet_to_hex(octet));
                    
                    continue;
                }

                let mut new_permutations = Vec::new();

                for permutation in folding_permutations {
                    new_permutations.push(format!("{}.{}", permutation, octet.to_string()));
                    new_permutations.push(format!("{}.{}", permutation, octet_to_octal(octet)));
                    new_permutations.push(format!("{}.{}", permutation, octet_to_hex(octet)));
                }

                folding_permutations = new_permutations;
            }

            folding_permutations
                .into_iter()
                .for_each(|permutation| possibilities.push(format!("{}.{}", permutation, decimal_value)));
        }

        possibilities
    }

    pub fn to_ipv6(&self) -> String {
        Ipv4Addr::new(self.octets[0], self.octets[1], self.octets[2], self.octets[3])
            .to_ipv6_mapped()
            .to_string()
    }
}

impl V6 {
    pub fn to_decimal(&self) -> u128 {
        self.octets
            .iter()
            .rev()
            .enumerate()
            .map(|(index, &octet)| { (octet as u128) << (index * 8) })
            .sum()
    }
}

fn octet_to_hex(octet: &u8) -> String {
    format!("0x{:x}", octet)
}

fn octet_to_octal(octet: &u8) -> String {
    format!("0{:o}", octet)
}

pub fn to_decimal(octets: &Vec<&u8>) -> u32 {
    octets
        .iter()
        .rev()
        .enumerate()
        .map(|(index, &&octet)| { (octet as u32) << (index * 8) })
        .sum()
}
