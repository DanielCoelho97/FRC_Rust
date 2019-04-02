fn main() {
    let vitor = Ator {"Vitor", 124356, 201210, 22/11/1992};
    let personagem = Personagem {"Astolfo", "Secundário", "Personagem idoso, que fica jogando dominó"};
    let filme = Filme {"A Origem", 06/08/2010}
    let textoPeca = TextoPeca {"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod 
    tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud 
    exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in
    reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint
    occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.", 
    filme, null, null}

    vitor.inserePapel(personagem);
    vitor.setTextoPeca(textoPeca);

}