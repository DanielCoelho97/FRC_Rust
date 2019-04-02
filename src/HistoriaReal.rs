struct HistoriaReal {
    let relato: String;
    let pessoa: Vec<Pessoa> = Vec::new();
};

impl HistoriaReal {
    fn origin(&mut self, relato: String, pessoa: Vec<Pessoa>) -> HistoriaReal {
        HistoriaReal { self.pessoa = pessoa, self.relato = relato }
    }

    fn inserePessoa(&mut self, pessoa: Pessoa) {
        self.pessoa.push(pessoa);
    }
    
}