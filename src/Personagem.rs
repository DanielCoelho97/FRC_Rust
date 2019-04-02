struct Personagem {
    let nomePersonagem: String;
    let tipo: String;
    let descricao: String;
};

impl Personagem {
    fn origin(&mut self, nomePersonagem: String, tipo: String, descricao: String) -> Personagem {
        Personagem { self.nomePersonagem = nomePersonagem, self.tipo = tipo }
    }

    fn getDescricao() -> String {
        return descricao;
    }

}