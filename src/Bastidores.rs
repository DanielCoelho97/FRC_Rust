struct Bastidores {
    parent: Pessoa;
    let mut funcao: String;
};

impl Bastidores {
    fn origin(&mut self, funcao: String) -> Bastidores {
        Bastidores { self.funcao = funcao }
    }
}

