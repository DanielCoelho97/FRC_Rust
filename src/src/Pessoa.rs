struct Pessoa {
    let nome: String;
    let cpf: String;
    let rg: String;
    let dataNasc: DateTime; 
    let mut textoPeca: TextoPeca;
};

impl Pessoa {
    fn origin(&mut self, nome: String, cpf: String, rg: String, dataNasc: DateTime) 
    -> Pessoa {
        Pessoa { self.nome = nome, self.cpf = cpf, self.rg = rg, self.dataNasc = dataNasc }
    }

    fn setTextoPeca(&mut self, textoPeca: TextoPeca) {
        self.textoPeca = textoPeca;
    }

    fn getTextoPeca() -> TextoPeca {
        return textoPeca;
    }

}

