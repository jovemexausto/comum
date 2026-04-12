# Apps

Projetos verticais mantidos pelo time do Comum.

Cada app pode reunir, no mesmo diretorio de projeto:

- superficie de app/UX
- capsulas locais usadas por esse fluxo
- simulacoes e fixtures
- manifests e schemas especificos do projeto
- testes e scripts de DX

Esses projetos vivem no mesmo monorepo, mas nao se confundem com a `spec/` do
protocolo.

Quando um projeto tiver capsulas proprias, o SSOT semantico local deve viver no
proprio slice do app, nao na `spec/` nem espalhado entre codigo e simulacao.

Cada slice tambem pode declarar, em `capsules.yaml`, quais packages capsulares
usa no momento.
