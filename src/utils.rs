use crate::store::Store;
use std::convert::Infallible;
use warp::Filter;

pub fn with_store(store: Store) -> impl Filter<Extract = (Store,), Error = Infallible> + Clone {
    warp::any().map(move || store.clone())
}

pub fn tv_shows() -> Vec<String> {
    vec![
        String::from("Breaking Bad"),
        String::from("Avatar: The Last Airbender"),
        String::from("Game of Thrones"),
        String::from("Fullmetal Alchemist: Brotherhood"),
        String::from("Attack on Titan"),
        String::from("The Office"),
        String::from("Arcane"),
        String::from("Better Call Saul"),
        String::from("Death Note"),
        String::from("Gravity Falls"),
        String::from("It's Always Sunny in Philadelphia"),
        String::from("One Piece"),
        String::from("South Park"),
        String::from("The Simpsons"),
        String::from("Spongebob Squarepants"),
        String::from("Stranger Things"),
        String::from("House of Cards"),
        String::from("Adventure Time"),
        String::from("Gilmore Girls"),
        String::from("Community"),
        String::from("Family Guy"),
        String::from("Scrubs"),
        String::from("How I met your Mother"),
        String::from("King of Queens"),
        String::from("Brooklyn 99"),
        String::from("Two and a Half Men"),
        String::from("The Prine of Bel-Air"),
        String::from("Alf"),
        String::from("Dr. House"),
        String::from("The Big Bang Theory"),
        String::from("Nightrider"),
        String::from("Baywatch"),
        String::from("Suits"),
        String::from("Drake and Josh"),
        String::from("Lost"),
        String::from("Gray's Anatomy"),
        String::from("Dexter"),
        String::from("Sex and the City"),
        String::from("Desprate Housewives"),
        String::from("Planet Earth"),
        String::from("Pokemon"),
        String::from("Zoey 11"),
        String::from("iCarly"),
        String::from("The Witcher"),
        String::from("Monk"),
        String::from("Cernobyl"),
        String::from("2 Broke Girls"),
        String::from("Unbreakable Kimmy Schmidt"),
        String::from("Private Practice"),
        String::from("Bones"),
        String::from("Timeless"),
        String::from("Crazy Ex-Girlfriend"),
    ]
}
