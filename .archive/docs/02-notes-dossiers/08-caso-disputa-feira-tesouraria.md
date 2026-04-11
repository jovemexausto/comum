# Caso concreto: disputa de tesouraria na Feira

Objetivo: modelar um conflito comunitario realista sem inflar o protocolo base.
Esta nota separa, de forma explicita, tres camadas:

- o que o core do Comum consegue validar estruturalmente;
- o que uma capsula de governanca/accountability precisaria decidir;
- o que continua dependendo de evidencia e julgamento institucional extra-
  protocolo.

Status: nota de trabalho. Nao normativa.

## 1. Cenario

Comunidade: Feira do Centro.

Atores:

- Maria: feirante.
- Joao: tesoureiro local.
- Carlos: mediador reconhecido pela comunidade.
- Ana e Bia: participantes que acompanharam parte dos fatos.

Contexto:

- A comunidade foi fundada por um par, depois cresceu.
- Joao administra uma pequena tesouraria local usada para manutencao da feira.
- Maria afirma que Joao desviou parte dos fundos para uso proprio.
- Joao nega o desvio e diz que o gasto foi emergencial e autorizado verbalmente.

Pergunta:

- Como o Comum organiza a auditabilidade dessa disputa sem fingir que o core,
  sozinho, resolve legitimidade substantiva?

## 2. O que o core consegue afirmar

Sem qualquer extensao institucional forte, o core ja consegue dizer:

- quais Testimonies existem;
- quem assinou cada Testimony;
- em que ordem causal minima eles se relacionam (`refs`, `prev_id`);
- se os payloads sao validos e canonicamente serializados;
- se houve revogacoes locais ou rotacoes de chave relevantes;
- se dois Commoners possuem o mesmo subgrafo e, portanto, a mesma base minima
  para leitura.

O core NAO consegue dizer sozinho:

- se o gasto foi legitimo;
- se a autorizacao verbal existiu;
- se Carlos tem jurisdicao suficiente para decidir;
- se a comunidade aceita a resolucao como justa.

## 3. Grafo minimo da disputa

Uma leitura minima do grafo poderia conter:

1. `genesis`
   - funda a comunidade e indica a capsula de governanca vigente.

2. `capsule/invoke` (`feira/offer`, `feira/accept`, etc.)
   - estabelece atividade economica normal da comunidade.

3. `comum/transfer` ou fluxo capsular equivalente
   - registra movimentacoes da tesouraria ou pagamentos relevantes.

4. `capsule/invoke` de governanca local
   - abre uma contestacao ou pedido de apuracao.

5. `capsule/invoke` / `capsule/result`
   - registra admissibilidade, escuta, decisao provisoria, escalacao ou fechamento.

## 4. Fluxo conceitual sugerido

Sem tornar isso normativo no core, um fluxo capsular plausivel seria:

### Etapa A: alegacao

Maria emite um Testimony de contestacao via capsula de governanca:

- alvo: Joao
- alegacao: desvio de recursos
- referencias: ids das movimentacoes contestadas
- pedido: apuracao e eventual suspensao temporaria do mandato

### Etapa B: resposta

Joao emite resposta:

- nega a alegacao
- referencia os mesmos ids
- aponta justificativa contextual
- junta, se houver, testemunhos de autorizacao previa

### Etapa C: admissibilidade / jurisdicao

A comunidade, ou um mediador reconhecido, registra se a contestacao:

- e admissivel
- pertence a sua jurisdicao
- exige escalacao

### Etapa D: decisao

Uma decisao capsular poderia conter:

- status: procedente / improcedente / inconclusiva / escalada
- fundamento curto
- referencias usadas
- consequencias:
  - suspensao de Joao
  - manutencao do mandato
  - reparacao material
  - nova rodada de coleta de evidencia

### Etapa E: revogacao ou restauracao

Se a comunidade entender que houve abuso, um fluxo local de revogacao pode ser
acionado. Se a acusacao cair, a comunidade pode registrar restauracao de
confianca ou encerramento sem sancao.

## 5. O que precisa ser capsula, nao core

Esta nota reforca que os seguintes elementos sao capsulares:

- quem pode abrir contestacao;
- quem decide admissibilidade;
- quem tem jurisdicao para resolver;
- qual quorum e necessario para sancao;
- quando escalacao e obrigatoria;
- que tipo de reparacao existe;
- como tesouraria e governada.

Se isso subisse para o core agora, o protocolo passaria a impor uma teoria de
governanca que ainda nao foi validada por casos reais.

## 6. O que permanece extra-protocolo

Mesmo com uma boa capsula de disputa, alguns elementos permanecem externos:

- prova testemunhal oral;
- filmagem, foto, recibo em papel;
- conhecimento local sobre urgencia, costume ou contexto afetivo;
- aceitacao social da decisao;
- cumprimento material da reparacao.

O Comum nao deve fingir autossuficiencia juridica. Seu ganho aqui e outro:

- legibilidade do processo
- encadeamento publico das alegacoes
- auditabilidade da decisao
- capacidade de revisao futura

## 7. Onde a disputa pode falhar

Falhas relevantes para futuras simulacoes:

- Joao controla conectividade e atrasa propagacao.
- Carlos e visto como parcial.
- Ana e Bia discordam entre si sobre o ocorrido.
- A comunidade se divide e nenhuma decisao alcança aceitacao suficiente.
- A reparacao decidida nao e cumprida.

Esses casos mostram que o protocolo nao substitui instituicao viva. Ele apenas
impede que a disputa desapareca sem rastro.

## 8. Hipoteses testaveis derivadas deste caso

### H1: o rastro melhora auditabilidade

Se a disputa for processada com cadeia publica de contestacao, resposta e
resolucao, terceiros da comunidade devem conseguir reconstruir melhor o caso do
que em um fluxo puramente verbal.

### H2: accountability capsular reduz arbitrariedade silenciosa

Se a capsula exigir resolucao com fundamento e referencias, o espaco para
arbitrariedade opaca deve diminuir.

### H3: o custo institucional continua real

Mesmo com bom rastro protocolar, o custo de mediar, decidir e fazer cumprir a
decisao continua sendo humano e institucional.

## 9. Criterios de falsificacao para a linha de pesquisa

Este caso comeca a falsificar a tese de autoridade emergente auditavel se,
mesmo com rastro completo:

- a comunidade nao consegue identificar quem decidiu o que;
- diferentes nós leem resultados estruturalmente incompatíveis;
- a capsula nao oferece mecanismo claro de revisao ou escalacao;
- o processo inteiro continua indistinguivel de uma autoridade informal opaca.

## 10. Proximo passo sugerido

Transformar este caso em simulacao conceitual ou RFC capsular, sem alterar o
core do protocolo:

- `mandate`
- `challenge`
- `resolution`
- `revoke`

Mas isso so deve subir de nivel quando houver mais evidencia de uso, custo e
capturabilidade.
