# Rasm

Rasm est un language approximatif, codé en rust et qui se compile en assmbleur x86_64.

Non, je ne connais ni le rust ni l'assembleur, mais oui je me lance la dedans, on verra bien

Inspiré de <https://gitlab.com/tsoding/porth>

## Syntaxe
`<int>` : push le nombre sur la stack
`.`     : pop la stack et affiche le nombre
`+`     : pop les deux derniers nombres de la stack, les additionne et push le résultat

## Objectifs long terme :
- [ ] Turing complet
- [ ] Accès mémoire correct
- [ ] Self hosting
- [ ] Type checking


## Objectifs proches :
- [ ] système d'erreurs
- [ ] tests qui lancent rasm et non pas l'output
- [ ] tests capable de gérer les erreurs
- [x] macros
- [ ] includes de fichiers
- [ ] mode interprété
- [x] mode compilé
- [ ] mode debug
- [x] syscalls
- [ ] allocate memory
- [ ] stdlib (refaire celle du C?)