* Jeux de Vie

		000x26=78     3X8=24 3X26=78 24*78 = 1872
		000   (1872 cases  0 ou 1)
		000
		x
		=
		8

* Liste des regles

		100     100     100     000     001
		001     010     010     110     111
		010     010     110     000     010
 		v       v       v       v       v
 		R1      R2      R2      R3     R3
		...     ...     ...     ...     ...
		.1.     .1.     .1.     ...     ...
		...     ...     ...     ...     ...

- R1 : une cellule morte possedant exactement trois cellules voisines vivantes, nait;
- R2 : une cellule vivante possedant deux ou trois cellules voisines vivantes le reste;
- R3 : une cellule vivante ne possedant pas deux ou trois cellules voisines vivantes meurt (par isolement ou par surpeuplement);
