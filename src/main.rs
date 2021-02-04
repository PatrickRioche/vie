// vie V0.0					Angers, le 24/01/2021
//
//  000x26=78	3X8=24 3X26=78 24*78 = 1872
//  000         1872 cases  0 ou 1
//  000
//   x
//   =
//   8
//
//	Liste des regles :
//	100	100	100	000	001
//	001	010	010	110	111
//	010	010	110	000	010
//	 v   v   v   v   v
//  R1  R2  R2  R3  R3
//	...	...	...	...	...
//	.1.	.1.	.1.	...	...
//	...	...	...	...	...
//
// R1 : une cellule morte possedant exactement trois cellules voisines vivantes, nait ;
// R2 : une cellule vivante possedant deux ou trois cellules voisines vivantes le reste ;
// R3 : une cellule vivante ne possedant pas deux ou trois cellules voisines vivantes meurt (par isolement ou par surpeuplement).
//
//#! [warn(dead_code)]


use std::env;
use std::io;
use std::process;


fn affiche_ecran( cel : [i8;1872] ) {

	let mut debut_ligne:usize 	= 0;
	let mut fin_ligne:usize   	= 77;

	for _cpt_ligne in 0..23 {
		affiche_ligne( debut_ligne , fin_ligne  , cel );
		debut_ligne += 77;
		fin_ligne   += 77;
	}
}

fn affiche_ligne( premier_car:usize, dernier_car:usize, cel : [i8;1872])  {

	for cpt_caractere in premier_car..dernier_car {
		
		if cel[cpt_caractere] == 0 {
			print!(" ");
		} else {
			print!("X");
		}
	}
	println!();

}

fn main () {
	//
	//  Recuperation des arguments dans la ligne de commande
	//
	let arguments: Vec<String> = env::args().collect();
	let mut _nombre_init:usize = 2;

    if arguments.len() == 1 {
		println!("vie.exe nombre_initialisation");
		return;
	}
	else {
		_nombre_init = arguments[1].parse::<usize>().unwrap();
	}
	
	println!("Argument = {} {}", arguments[0], arguments[1]);

	let  mut cellule:   [ i8 ; 1872 ] = [0 ; 1872];
	//let  mut cellulem1: [ i8 ; 1872 ] = [0 ; 1872];

	//
	//	initialisation de cellule avec de 0 ou 1 en fonction du reste de la division par 2 ou 3, 5, 7, 9
	//
	for i in 0..1872 {
		if ( i % _nombre_init ) == 0  {
			cellule[i] = 0;
		} else {
			cellule[i] = 1;
		}
	}

	println!(" Affichage des cellules :");

	let mut evolution:i8 = 1;
	
	loop {
		
        assert!( evolution < 127, "Trop d'evolution = {}", evolution); 
			
		affiche_ecran( cellule );

		let mut input = String::new();

		match io::stdin().read_line(&mut input) {
			Ok(_n) => {
				//println!("{} bytes read", n);
				//println!("input ={}", input);
			}
			Err(error) => println!("error: {}", error),
		};


		//let  hello = "Hello Word!";
		//let result = &hello[1..4];
		//println!("Method 1 {}", result);
		//let result = hello.get(1..4);
		//println!("Method 2 {:?}", result);

		//
		//	Decale les cellules de 1
		//
		let cellule0:i8 ;
		cellule0 = cellule[0];
		for j in 1..1871 { cellule[j-1]=cellule[j]; }
		cellule[1871] = cellule0;
		
		
		//
		//	Analyse des regles
		//
		let mut _voisin:i8 = 0;
		let mut centre:usize = 4;
		
		loop {
			
			_voisin = 0;
			
			if cellule[centre-4]==1 { _voisin = _voisin + 1; }
			if cellule[centre-3]==1 { _voisin = _voisin + 1; }
			if cellule[centre-2]==1 { _voisin = _voisin + 1; }
			if cellule[centre-1]==1 { _voisin = _voisin + 1; }
			if cellule[centre+1]==1 { _voisin = _voisin + 1; }
			if cellule[centre+2]==1 { _voisin = _voisin + 1; }
			if cellule[centre+3]==1 { _voisin = _voisin + 1; }
			if cellule[centre+4]==1 { _voisin = _voisin + 1; }
			
			//
			//	
			//
			if _voisin == 3  { 
				cellule[centre]=1; 
				}
			if  _voisin < 2 || _voisin > 3  { 
				cellule[centre]=0;	
				}
				
			centre = centre + 9;
			if centre > 1872 { break; }
			
		}
		
		//
		//	Test si le caractere q a ete saisir au clavier
		//
		for n in input.chars() {
				//println!("{}",n);
				if n == 'q' { process::exit(9); }
		}
		
		//
		//	Affiche le numero de l'evolution des cellules
		//
		println!("Evolution : {}", evolution);
		evolution = evolution + 1;
		
	}

}
