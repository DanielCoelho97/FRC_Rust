struct TextoPeca {
    let enredo: String;
    let filme: Filme;
    let livro: Livro;
    let historiaReal: HistoriaReal;
};

impl TextoPeca {
    fn origin(&mut self, enredo: String, filme: Filme, livro: Livro, historiaReal: HistoriaReal ) 
    -> TextoPeca {
        TextoPeca { self.enredo = enredo, self.filme = filme, self.livro = livro, self.historiaReal = 
        historiaReal }
    }

    fn getEnredo() {
        return enredo; 
    }

    fn getLivro() -> Livro {
        if livro != null {
            return livro;
        }else {
            print!("Essa peça não foi baseada em nenhum livro");
        }
        return null;
    }

    fn getFilme() -> Filme {
        if filme != null {
            return filme;
        }else {
            print!("Essa peça não foi baseada em nenhum filme");
        }
        return null;
    }

    fn getHistoriaReal() -> HistoriaReal {
        if historiaReal != null {
            return historiaReal;
        }else {
            print!("Essa peça não foi baseada em nenhuma historia real");
        }
        return null;
    }
    
}