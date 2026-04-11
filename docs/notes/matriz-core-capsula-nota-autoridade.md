# Matriz: core, capsula e nota para autoridade emergente

Objetivo: evitar inflacao semantica do protocolo base. Esta nota organiza o
tema de autoridade emergente em tres niveis:

- core: o que precisa estar no protocolo base para interoperabilidade minima
- capsula: o que deve ser decidido por semantica local de governanca
- nota: o que ainda e hipotese, pesquisa ou framing conceitual

## Regra de ouro

- Se algo e necessario para interoperabilidade minima, entra no core.
- Se algo define como uma comunidade governa, delega ou resolve disputa, tende
  a viver em capsula.
- Se algo ainda depende de evidencia, simulacao ou refinamento conceitual, fica
  em nota.

## 1. Core

O que faz sentido manter no protocolo base agora:

- `Testimony` como unidade atomica verificavel.
- grafo causal local + sync eventual.
- `claim.verb + claim.payload` como mecanismo de extensao.
- `context + proof` como separacao entre semantica e justificativa.
- identidade local e revogacao local, sem autoridade global.
- `Genesis` com minimo de 2 fundadores.
- regra normativa para par fundador: `N=2 -> threshold 2-of-2`.
- possibilidade de revogacao e rotacao locais como mecanismos genericos.
- linguagem geral de autoridade local auditavel, limitada e revogavel.

O que o core pode afirmar sem inflar demais:

- O protocolo nao elimina autoridade.
- O protocolo torna rastreavel a producao local de autoridade.
- O protocolo nao decide sozinho legitimidade substantiva.

O que o core NAO deve fixar agora:

- estagios formais de comunidade (`dyad`, `triad`, `plural`) como estrutura
  normativa obrigatoria.
- gating normativo de capacidades por escala.
- teoria fechada de disputa, jurisdicao ou sancao.

## 2. Capsula

O que deve viver em capsulas de governanca ou modulos semanticos locais:

- mandato com escopo.
- contestacao (`challenge`).
- resolucao de disputa.
- escalacao de jurisdicao.
- controle de tesouraria.
- criterios de quorum por contexto.
- regras de expulsao, suspensao, reparacao e restauracao.
- accountability explicita por dominio.
- thresholds diferenciados por tipo de decisao.

Exemplos:

- uma capsula Agora pode exigir 3+ validadores para certas decisoes.
- uma comunidade pequena pode permitir operacao cotidiana em diade, mas exigir
  triangulacao para dispute resolution.
- uma capsula de tesouraria pode bloquear controle individual mesmo em comunidade
  fundada a dois.

Formula pratica:

- o core reconhece que a comunidade existe;
- a capsula decide o que essa comunidade pode fazer com seguranca e legitimidade.

## 3. Nota / Pesquisa

O que deve permanecer como nota, pesquisa ou hipotese por enquanto:

- teoria triadica da coerencia como fundamento interpretativo forte.
- stage-based capabilities como modelo geral.
- testimony formal de triangulacao/transicao de fase.
- metricas de captura institucional.
- hipoteses sobre confianca antes/depois da entrada do terceiro participante.
- comparacoes com Ostrom, Alberoni, Jo Freeman, teoria dos jogos, etc.
- tese completa de "autoridade emergente auditavel" como programa de pesquisa.

Esses elementos podem orientar design e simulacao, mas nao devem dominar a
norma antes de evidencia melhor.

## 4. Heuristica de decisao

Perguntas para decidir onde algo pertence:

1. Sem isso, duas implementacoes deixam de interoperar?
   - Se sim, tende a ser core.
2. Isso define como uma comunidade governa um caso concreto?
   - Se sim, tende a ser capsula.
3. Isso ainda depende de validacao empirica ou e uma leitura teorica forte?
   - Se sim, tende a ser nota.

## 5. Aplicacao ao estado atual do repo

### Fica no core agora

- Genesis 2+
- default 2-of-2 para N=2
- framing de autoridade auditavel em docs centrais
- revogacao local como principio geral

### Nao sobe para o core agora

- `community-stage`
- `triangulation testimony`
- capabilities estratificadas por estagio
- `mandate/challenge/resolution/revoke` como pacote normativo completo

### Candidatos para capsula / RFC futura

- Agora como laboratorio de accountability
- semantica de mandato
- semantica de challenge/resolution
- fluxo de disputa em tesouraria/comercio/mutirao

## 6. Formula sintetica

- Core: torna poder legivel.
- Capsula: decide como poder opera.
- Nota: investiga por que essa forma pode funcionar.

## 7. Status

Nota de escopo e higiene conceitual. Nao normativa.
