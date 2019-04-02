struct Autor {
    parent: Pessoa;
    let mut obrasFilme: Vec<Filme> = Vec::new();
    let mut obrasLivro: Vec<Livro> = Vec::new();
    let mut obrasPeca: Vec<TextoPeca> = Vec::new();
};

impl Autor {
    fn origin(&mut self, filmes: Vec<Filme>, livros: Vec<Livro>, pecas: Vec<TextoPeca>) -> Autor {
        Autor { self.obrasFilme = f, self.obrasLivro = l, self.obrasPeca = p }
    }

    fn insereLivro(&mut self, livro: Livro) {
        self.obrasLivro.push(livro);
    }

    fn insereFilme(&mut self, filme: Filme) {
        self.obrasFilme.push(filme);
    }

    fn inserePeca(&mut self, peca: Peca) {
        self.obrasPeca.push(peca);
    }

    fn getLivros() -> Vec<Livro> {
        return self.obrasLivro;
    }

    fn getFilmes() -> Vec<Filme> {
        return self.obrasFilme;
    }

    fn getPecas() -> Vec<TextoPeca> {
        return self.obrasPeca;
    }

}