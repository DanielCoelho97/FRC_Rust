struct Livro {
    let nome: String;
    let lancamento: DateTime;
};

impl Livro {
    fn origin(&mut self, nome: String, lancamento: DateTime) -> Livro {
        Livro { self.nome = nome, self.lancamento = lancamento }
    }   

    fn getNome() -> String {
        return nome;
    }

    fn getLancamento() -> String {
        return lancamento;
    }

}