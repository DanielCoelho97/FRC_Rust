struct EventoPeca {
    let mut local: String;
    let mut duracao: f64;
    let mut pessoas: Vec<Pessoa> = Vec::new();
    let textoPeca: TextoPeca; 
};

impl EventoPeca {
    fn origin(&mut self, local: String, duracao: f64, pessoas: Vec<Pessoa>, textoPeca: TextoPeca) 
    -> EventoPeca {
        EventoPeca { self.local = local, self.duracao = duracao, self.pessoas = pessoas, self.textoPeca = 
        textoPeca }
    }

    fn inserePessoa(&mut self, pessoa: Pessoa) {
        self.pessoas.push(pessoa);
    }

    fn getTextoPeca() -> TextoPeca {
        return textoPeca;
    }
}