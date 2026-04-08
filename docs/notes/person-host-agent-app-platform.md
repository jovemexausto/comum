# Person, Host, Agent, App, Platform (nota)

Objetivo: fixar uma semantica de camadas para pensar identidade, execucao,
agencia, interface e infraestrutura sem colapsar pessoa em app, app em
plataforma, ou comunidade em stack contingente.

Tudo abaixo e nao normativo.

## Ver tambem

- [Humano, rastro e legitimidade](./humano-rastro-e-legitimidade.md)
- [Mapa transversal de tensoes estruturais](./mapa-transversal-tensoes-estruturais.md)
- [Dump lossless: verticalidade, mal, cancer e campos vivos](./dump-lossless-verticalidade-mal-cancer-e-campos-vivos.md)

## 1. Ponto de partida

Um erro recorrente em arquiteturas digitais e colapsar camadas distintas em um
mesmo objeto pratico.

Exemplos comuns:

- tratar o app como se fosse a identidade
- tratar a plataforma como se fosse a comunidade
- tratar o dispositivo como se fosse a soberania da pessoa
- tratar a interface como se fosse a fonte de legitimidade

Esses colapsos tornam a arquitetura mais simples no curto prazo, mas tambem:

- aumentam dependencia estrutural
- reduzem portabilidade
- confundem pertencimento com uso de produto
- entregam soberania a intermediarios contingentes

Por isso, a semantica correta precisa separar com rigor:

- pessoa
- hospedeiro
- agente
- app
- plataforma

## 2. As cinco camadas

### Person

`Person` e a fonte de legitimidade.

Nao e interface.
Nao e dispositivo.
Nao e credencial.
Nao e plataforma.

E o polo que:

- autoriza
- assume compromisso
- arbitra conflito
- responde etica e politicamente por suas acoes

Em formulacao resumida:

> `Person` e a fonte de legitimidade, consentimento e responsabilidade.

## 3. Host

`Host` e o ambiente local de execucao.

Pode ser:

- telefone
- laptop
- desktop
- servidor local
- daemon box
- qualquer ambiente futuro capaz de executar o agente

O `Host` importa porque torna a agencia operacional em uma situacao concreta.
Mas ele nao deve ser confundido com a identidade nem com a comunidade.

Em formulacao resumida:

> `Host` e o ambiente local que abriga e roda o `Agent`.

## 4. Agent

`Agent` e a instancia local de agencia da `Person`.

Aqui, `Agent` nomeia o `Commoner` como forma operacional de continuidade,
contexto e capacidade de agir.

O `Agent`:

- roda em um `Host`
- age em nome da `Person`
- guarda continuidade local
- aplica politicas e consentimentos
- negocia capacidades com apps
- preserva portabilidade entre interfaces

Ele nao e a `Person`, mas tambem nao e mero app.
E a camada que traduz legitimidade pessoal em acao situada.

Formula mais precisa:

> O Agent nao substitui a Person; ele opera como sua instancia local de agencia.

Ou, de forma mais descritiva:

> `Agent` representa a `Person`, guarda continuidade local e medeia acesso a
> capacidades.

## 5. App

`App` e a superficie especializada de interacao.

Um `App` pode oferecer:

- UX
- fluxos sociais
- visualizacoes
- semanticas locais
- rituais de uso
- experiencias especializadas de comunidade

Mas ele nao deve ser tratado como o dono da identidade.
Ele conversa com o `Agent`.
Ele solicita capacidades ao `Agent`.
Ele torna visivel um modo especifico de participar.

Em formulacao resumida:

> `App` e a interface especializada pela qual uma forma situada de participacao
> se torna visivel e habitavel.

## 6. Platform

`Platform` e a infraestrutura contingente onde `Host` e `App` existem em um dado
momento historico.

Exemplos:

- iOS
- Android
- browser stacks
- app stores
- clouds e vendors atuais

A `Platform` pode ser usada.
Mas nao deve ser tratada como fundamento da comunidade, da identidade ou da
agencia.

Ou, em formulacao politica mais explicita:

> A Platform e substrato contingente, nao fonte de legitimidade.

## 7. Encadeamento correto

A ordem semantica recomendada e esta:

1. `Person` e a fonte de legitimidade, consentimento e responsabilidade
2. `Agent` representa a `Person`, guarda continuidade e medeia acesso a capacidades
3. `Host` abriga e roda o `Agent`
4. `App` oferece uma interface situada de interacao e participacao
5. `Platform` fornece a infraestrutura contingente sobre a qual isso existe hoje

Em formula condensada:

> The Person is the source of legitimacy.
>
> The Agent represents the Person and mediates access to capabilities.
>
> The Host runs the Agent locally.
>
> The App provides a situated interface for participation.
>
> The Platform is contingent infrastructure.

Ou em portugues:

> A Pessoa e a fonte de legitimidade.
>
> O Agente representa a Pessoa e medeia acesso a capacidades.
>
> O Host abriga e roda o Agente localmente.
>
> O App oferece uma interface situada de participacao.
>
> A Plataforma e infraestrutura contingente.

## 8. O que essa semantica evita

### 8.1 Identidade colapsada em app

Sem essa separacao, trocar de app pode parecer trocar de identidade.

Com essa separacao:

- a identidade permanece ligada a `Person` e `Agent`
- apps passam a ser modos de acesso, nao donos do sujeito

### 8.2 Comunidade colapsada em plataforma

Sem essa separacao, parece que uma comunidade "mora" em iOS, Android ou em uma
loja de apps.

Com essa separacao:

- a comunidade pode atravessar plataformas
- a plataforma vira apenas suporte historico provisorio

### 8.3 Soberania colapsada em dispositivo

Sem essa separacao, parece que o hardware em si e a fonte da identidade.

Com essa separacao:

- o `Host` importa operacionalmente
- mas a continuidade semantica esta no nexo entre `Person` e `Agent`

### 8.4 Interface colapsada em agencia

Sem essa separacao, o app aparece como sujeito da acao.

Com essa separacao:

- o app pede
- o agente decide e age
- a pessoa legitima

## 9. Consequencia filosofica

Essa semantica protege uma distincao ontologica importante:

- quem aparece nao e identico ao meio de aparicao
- quem age nao e identico a superficie de interacao
- quem legitima nao e identico ao mecanismo de execucao

Isso importa porque sistemas tecnicos frequentemente naturalizam suas interfaces,
como se o que aparece na tela fosse o proprio sujeito.

Esta nota recusa essa confusao.

Formula forte:

> O meio de acesso nao deve reivindicar o lugar da fonte de legitimidade.

## 10. Consequencia politica

Politicamente, essa semantica distribui soberania de modo menos colonial.

Ela afirma:

- a pessoa nao pertence ao app
- a comunidade nao pertence a plataforma
- a interface nao deve sequestrar a agencia
- a infraestrutura nao deve fingir que funda o pertencimento

Isto permite uma estrategia realista e nao ingenua:

- usar plataformas atuais sem depender delas como destino final
- operar sobre stacks dominantes sem entregar a elas o centro normativo
- construir continuidade para alem do involucro tecnico do momento

Formula politica:

> Usamos plataformas contingentes sem lhes conceder soberania.

## 11. Consequencia de DX e implementacao

Essa semantica tambem melhora desenho de produto e implementacao.

Ela permite pensar:

- multiplos apps sobre o mesmo `Agent`
- CLI, mobile, web e outros clientes como superficies equivalentes em dignidade
- migracao de interface sem perda de continuidade
- politicas locais no `Agent`, em vez de espalhadas por apps
- hospedeiros diversos sem redefinicao do sujeito

Em vez de perguntar:

- "em qual app a pessoa existe?"

A pergunta correta passa a ser:

- "como diferentes apps acessam, com consentimento, a mesma agencia local?"

## 12. Formula final

Uma sintese possivel:

> A Person e a fonte de legitimidade.
>
> O Agent e a instancia local que representa a Person, guarda continuidade e
> medeia acesso a capacidades.
>
> O Host e o ambiente local que abriga e roda o Agent.
>
> O App e a superficie especializada de interacao.
>
> A Platform e a infraestrutura contingente que hoje carrega essas camadas.

Outra formulacao:

> A identidade nao mora no app.
>
> A comunidade nao mora na plataforma.
>
> A continuidade passa pela relacao entre Person e Agent, executada em Hosts e
> acessada por Apps.

E uma ultima, mais curta:

> Person grounds legitimacy.
>
> Agent represents and mediates.
>
> Host houses and runs.
>
> App interfaces.
>
> Platform contingently supports.
