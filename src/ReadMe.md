# TP2 Rust : Amazing Maze

*L'objectif de ce TP est d'aborder les notions de pointeurs intelligents, des types algébriques et de concurrence en Rust.*  
*Pour cela, nous avons implémenté plusieurs applications afin d'explorer un labyrinthe*

**Auteur** : Pierre Lafon et Jules Carpio   
**Professeur** : Jacques Noyé  
**Destination** : Mont Emei (Métaphore de la pratique)  
**Nom du cours** : Programmation multi-paradigme avec Rust  
**Date mise à jour** : 09/02/2023

Nous avons développé deux fichiers mains qui permettent de tester les deux premières approches.

### 3- Passage à Rust :

Nous retrouvons dans le fichier ```main3.rs``` :
- L'enumération de nos deux niveaux d'explorations
```rust 
enum Exploration {
  Explored,
  UnExplored,
}
```
- L'énumération de nos types : branche et feuille 
```rust 
enum Maze {
  Branch {
    label: String,
    left: Rc<RefCell<Maze>>,
    right: Rc<RefCell<Maze>>,
    status: Exploration,
  },
  Leaf { label: String },
}
```
Nous avons utilisé les pointeurs ```rc``` étant donné que les deux branches sont partagées  
- La structure de notre labyrinthe et son implémentation qui prend en compte les pointeurs intelligents afin de gérer les éléments mutables
- L'implémentation de la fonction ```explore(maze: &Rc<RefCell<Maze>>, trace: &mut Vec<String>)``` :
```rust
   fn explore(maze: &Rc<RefCell<Maze>>, trace: &mut Vec<String>) {
        let mut maze = maze.borrow_mut();
        match &mut *maze {
            Maze::Branch { label, left, right, status } => {
                trace.push(label.clone());
                match status {
                    Exploration::Explored => {}
                    Exploration::UnExplored => {
                        *status = Exploration::Explored;
                        explore(left, trace);
                        explore(right, trace);
                    }
                }
            }
            Maze::Leaf { label } => {
                trace.push(label.clone());
            }
        }
    }
```
Fonction similaire à celle développée en scala.  
Notons la récursivité si la branche n'a pas été explorée.  
```&mut``` Nous permet de gérer plus facilement les vecteurs en ayant la possibilité de directement ```push``` un élément dans la liste sans devoir gérer la chaîne de pointeur.

### 4- Version intermédiaire : 

Pour cette partie nous allons aborder les modifications apporter à la partie 3.
Soit nous retrouvons dans le fichier ```main4.rs``` :
- L'énumération d'un nouveau niveau d'exploration :
```rust
enum Exploration {
    Explored,
    UnExplored,
    PartiallyExplored, //Branche gauche explorée
}
```
- La nouvelle implémentation de ```explore``` :
```rust
    impl Maze {
    fn explore(&self, node: Rc<Self>, work: &mut Vec<Rc<Self>>, trace: &mut Vec<String>) {
        match self {
            Maze::Leaf { label } => {
                trace.push(label.clone());
            }
            Maze::Branch { label, left, right, status } => {
                trace.push(label.to_owned());
                let mut status = status.borrow_mut();
                match *status {
                    Exploration::Explored => {}
                    Exploration::PartiallyExplored => {
                        *status = Exploration::Explored;
                        work.push(right.clone());
                    }
                    Exploration::UnExplored => {
                        *status = Exploration::PartiallyExplored;
                        work.push(node.clone());
                        work.push(left.clone());
                    }
                }
            }
        }
    }
}
```
On retrouve notre stack ```work``` contenant au début la racine de l'arbre.  
Puis, en fonction du statut de la branche, ```work``` va se voir ajouter de nouveaux éléments à explorer.  
On retrouve notre ```PartiallyExplored``` qui permet d'ajouter la partie de droite aux éléments à explorer.  
Nous utilisons toujours des ```&mut``` afin de faciliter l'utilisation des vecteurs.
Nous obtenons donc étape par étape l'exploration du labyrinthe.

### 5- Version concurrente (bonus)

Nous n'avons pas eu le temps de développer la version concurrente.
