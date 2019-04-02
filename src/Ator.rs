struct Ator {
    parent: Pessoa;
    let mut fala: Vec<String> = Vec::new();
    let mut papel: Vec<Personagem> = Vec::new();
};

impl Ator {
    fn origin(&mut self, fala: Vec<String>, papel: Vec<Personagem>) -> Ator {
        Ator { self.fala= fala, self.papel= papel }
    }

    fn inserePapel(personagem: Personagem) {
        papel.push(personagem);
    }

}