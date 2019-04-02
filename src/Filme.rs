struct Filme {
    let nome: String;
    let lancamento: DateTime;
};

impl Filme {
    fn origin(&mut self, nome: String, lancamento: DateTime) -> Filme {
        Filme { self.nome = nome, self.lancamento = lancamento }
    }

    fn getNome() -> String {
        return nome;
    }

    fn getLancamento() -> DateTime {
        return lancamento;
    }
    
}