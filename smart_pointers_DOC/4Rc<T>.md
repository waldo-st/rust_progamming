## Rc<T>, le Smart Pointer avec Comptage de Références
Dans la majorité des cas, la possession est claire : vous savez exactement quelle variable possède une valeur donnée. Cependant, il y a des cas où une seule valeur peut avoir plusieurs propriétaires. Par exemple, dans les structures de données en graphe, plusieurs arêtes peuvent pointer vers le même nœud, et ce nœud est conceptuellement possédé par toutes les arêtes qui pointent vers lui. Un nœud ne devrait pas être nettoyé tant qu'il n'a pas d'arêtes pointant vers lui, donc il n'a plus de propriétaires.

Vous devez activer explicitement la possession multiple en utilisant le type Rust `Rc<T>`, qui est une abréviation pour comptage de références. Le type `Rc<T>` garde une trace du nombre de références à une valeur pour déterminer si la valeur est toujours utilisée. S'il n'y a aucune référence à une valeur, la valeur peut être nettoyée sans que des références ne deviennent invalides.

Imaginez `Rc<T>` comme une télévision dans une salle de séjour familiale. Quand une personne entre pour regarder la télévision, elle l'allume. D'autres peuvent entrer dans la salle et regarder la télévision. Quand la dernière personne quitte la salle, elle éteint la télévision parce qu'elle n'est plus utilisée. Si quelqu'un éteint la télévision pendant que d'autres la regardent encore, il y aurait un tollé parmi les téléspectateurs restants !

Nous utilisons le type` Rc<T>` lorsque nous voulons allouer des données sur le tas pour que plusieurs parties de notre programme puissent les lire et que nous ne pouvons pas déterminer à la compilation quelle partie finira d'utiliser les données en dernier. Si nous savions quelle partie finirait en dernier, nous pourrions simplement faire de cette partie le propriétaire des données, et les règles de possession normales appliquées à la compilation prendraient effet.

Notez que `Rc<T>` est uniquement destiné à être utilisé dans des scénarios à un seul thread. Lorsque nous aborderons la concurrence au chapitre 16, nous expliquerons comment effectuer le comptage de références dans des programmes multithread.

## Utiliser `Rc<T>` pour Partager des Données
Revenons à notre exemple de liste chaînée dans la Liste 15-5. Rappelons que nous l'avons définie en utilisant `Box<T>`. Cette fois-ci, nous allons créer deux listes qui partagent toutes deux la possession d'une troisième liste. Conceptuellement, cela ressemble à la Figure 15-3 :


Voici comment nous pouvons utiliser `Rc<T>` pour créer cette structure de liste :

## Exemple de Code avec `Rc<T>`
```
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
```
## Explication du Code
- Nous avons défini une énumération List avec deux variantes : Cons, qui contient un entier et une référence comptée à une autre List, et Nil, qui représente la fin de la liste.
- Dans main, nous créons une instance de Rc<List> contenant Cons(5, Rc::new(Cons(10, Rc::new(Nil)))).
- Ensuite, nous créons deux nouvelles listes, b et c, qui partagent toutes deux la possession de la liste a en utilisant Rc::clone(&a). Cela augmente le nombre de références à a.
## Comportement de Rc<T>
Lorsque nous clonons a en b et c, nous n'effectuons pas une copie en profondeur de a. Au lieu de cela, nous augmentons le compteur de référence de a. Ainsi, a est partagé entre b et c. Rust gardera une trace de ces références et nettoiera la mémoire de a une fois que toutes les références auront été supprimées.

L'utilisation de Rc<T> permet de partager en toute sécurité la possession d'une valeur entre plusieurs parties de votre programme. Cela est particulièrement utile pour les structures de données comme les graphes ou pour les scénarios où vous ne pouvez pas déterminer à la compilation quelle partie de votre programme finira d'utiliser les données en dernier.
## Rc<T>, le Smart Pointer avec Comptage de Références
#### Problème de la Possession Unique
Dans la majorité des cas, la possession est claire : vous savez exactement quelle variable possède une valeur donnée. Cependant, il y a des cas où une seule valeur peut avoir plusieurs propriétaires. Par exemple, dans les structures de données en graphe, plusieurs arêtes peuvent pointer vers le même nœud, et ce nœud est conceptuellement possédé par toutes les arêtes qui pointent vers lui. Un nœud ne devrait pas être nettoyé tant qu'il n'a pas d'arêtes pointant vers lui, donc il n'a plus de propriétaires.

Nous devons activer explicitement la possession multiple en utilisant le type Rust Rc<T>, qui est une abréviation pour comptage de références. Le type Rc<T> garde une trace du nombre de références à une valeur pour déterminer si la valeur est toujours utilisée. S'il n'y a aucune référence à une valeur, la valeur peut être nettoyée sans que des références ne deviennent invalides.

## Utiliser Rc<T> pour Partager des Données
Imaginons que nous ayons une liste chaînée et que nous souhaitions que deux listes différentes partagent une sous-liste commune. Nous ne pouvons pas le faire avec Box<T> car Box<T> implique une possession unique. Voici comment Rc<T> peut nous aider.

Définition avec Box<T> (ne fonctionne pas)
```
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    let c = Cons(4, Box::new(a));
}
```
En compilant ce code, nous obtenons l'erreur suivante :
```
error[E0382]: use of moved value: `a`
```
La variante `Cons` possède les données qu'elle contient, donc lorsque nous créons la liste b, a est déplacé dans b et b possède a. Ensuite, lorsque nous essayons d'utiliser a à nouveau pour créer c, nous ne sommes pas autorisés car a a été déplacé.

### Définition avec Rc<T> (fonctionne)
```
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
```
## Explication du Code
- Nous avons défini une énumération List avec deux variantes : Cons, qui contient un entier et un Rc<List>, et Nil, qui représente la fin de la liste.
- Dans main, nous créons une instance de Rc<List> contenant Cons(5, Rc::new(Cons(10, Rc::new(Nil)))).
- Ensuite, nous créons deux nouvelles listes, b et c, qui partagent toutes deux la possession de la liste a en utilisant Rc::clone(&a). Cela augmente le nombre de références à a.
### Suivi du Comptage de Références
Nous pouvons suivre le comptage de références avec Rc::strong_count :
```
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
```
## Sortie du Programme
```
count after creating a = 1
count after creating b = 2
count after creating c = 3
count after c goes out of scope = 2
```
Nous pouvons voir que le `Rc<List>` dans `a` a un compteur de référence initial de 1. Chaque fois que nous appelons `clone`, le compteur augmente de 1. Lorsque `c` sort du scope, le compteur diminue de 1. Nous n'avons pas besoin d'appeler une fonction pour diminuer le compteur de références comme nous devons appeler `Rc::clone` pour augmenter le compteur de références : l'implémentation de `Drop` diminue le compteur de références automatiquement lorsqu'une valeur `Rc<T>` sort du scope.
Ce que nous ne pouvons pas voir dans cet exemple, c'est que lorsque `b` puis a sortent de portée à la fin de `main`, le compteur est alors de 0, et le `Rc<List>` est complètement nettoyé. L'utilisation de` Rc<T>` permet à une seule valeur d'avoir plusieurs propriétaires, et le compteur garantit que la valeur reste valide tant que l'un des propriétaires existe toujours.

Grâce aux références immuables, `Rc<T>` vous permet de partager des données entre plusieurs parties de votre programme pour une lecture seule. Si `Rc<T>` vous permettait également d'avoir plusieurs références mutables, vous pourriez violer l'une des règles d'emprunt discutées au chapitre 4 : plusieurs emprunts mutables au même endroit peuvent entraîner des courses de données et des incohérences. Mais pouvoir muter des données est très utile ! Dans la prochaine section, nous discuterons du modèle de mutabilité interne et du type `RefCell<T>` que vous pouvez utiliser en conjonction avec un `Rc<T>` pour travailler avec cette restriction d'immutabilité.