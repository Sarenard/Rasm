# Rasm

Rasm est un language approximatif, codé en rust et qui se compile en assmbleur x86_64.

Non, je ne connais ni le rust ni l'assembleur, mais oui je me lance la dedans, on verra bien  

### Comment lancer rasm ?
Pour compiler et lancer `input/input.pyasm`
```bash
cargo build && ./target/debug/rasm -f input/input.pyasm && ./output/output
```
Pour lancer les tests
```bash
python3 test.py -t
```
Pour enregistrer de nouveaux tests
```bash
python3 test.py -r
```
Pour juste compiler rasm
```bash
cargo build --release
```

## Objectifs long terme :
- [ ] Turing complet
- [x] Accès mémoire correct
- [ ] Self hosting
- [ ] Type checking


## Objectifs proches :
- [ ] système d'erreurs
- [ ] tests qui lancent rasm et non pas l'output
- [ ] tests capable de gérer les erreurs
- [x] macros
- [x] includes de fichiers
- [ ] mode interprété
- [x] mode compilé
- [ ] mode debug
- [x] syscalls
- [x] système de mémoire
- [ ] stdlib (refaire celle du C?)
- [x] découper rasm en plusieurs fichiers
- [ ] Détailler plus la syntaxe dans le readme
- [ ] Ignorer les commentaires
- [ ] Syscall => push eax/rax
- [ ] Over

## Syntaxe
#### Découpage
La syntaxe sera découpée en trois morceaux : 
- L'explication de la commande
- Un cas d'utilisation dans pyasm
- Un équivalent en python

### `<int>`
Push le nombre donné sur la stack
```dart
3
```
```py
stack.push(3)
```
### `.`
Affiche le nombre sur la stack en décimal
```dart
3 .
```
```py
stack.push(3)
print(stack.pop())
```
### `+`
Additionne les deux éléments sur la stack et push le résultat
```dart
3 3 +
```
```py
stack.push(stack.pop() + stack.pop())
```

## Inspirations et remerciments

- [Porth](https://gitlab.com/tsoding/porth) Pour l'idée d'un language compilé en assembleur
- [@tsodingDaily](https://www.youtube.com/@TsodingDaily) Pour m'avoir donné envie de créer un language, puis un autre, puis un autre...
- [@rchapman](https://blog.rchapman.org/posts/Linux_System_Call_Table_for_x86_64/) Pour ce magnifique blog sur les syscall linux