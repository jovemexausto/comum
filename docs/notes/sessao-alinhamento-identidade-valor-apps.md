# Sintese de alinhamento da sessao

Objetivo: registrar os pontos que ficaram firmes nesta conversa para orientar teoria, spec e codigo nas proximas etapas.

## 1. Valor e emissao

- `Comum` deve ser nome do protocolo/projeto, nao nome automatico de moeda.
- A capacidade de emitir valor pertence a comunidades situadas, nao ao protocolo como centro.
- O protocolo nao deve totalizar uma equivalencia unica de valor.
- A federacao deve operar como camada fina de borda, nao como soberania superior.
- A unidade local de valor, se existir, deve ser definida pela comunidade ou pela capsula, nao herdada do nome do protocolo.

## 2. Comunidade

- Comunidade nao e simples container fisico.
- Vale separar comunidade nominal de comunidade funcional.
- Comunidade nominal pode ancorar identidade e pertencimento.
- Comunidade funcional pode emergir do grafo e das relacoes, sem Genesis compartilhado.
- A leitura da comunidade e local e contestavel.

## 3. Apps, capsulas e protocolo

- App nao e a comunidade.
- App e uma superficie de uso, automacao e UX sobre o protocolo.
- Capsula nao e a comunidade.
- Capsula e semantica executavel local, sob regras delimitadas.
- O protocolo define forma minima e interoperabilidade, nao a vida social inteira.

## 4. Commoner e node

- `Commoner` deve ser tratado como fachada tecnica de no, nao como sujeito total.
- O nodo runtime e a interface tecnica que valida, ingere, emite e sincroniza.
- O uso social do termo nao deve se confundir com a API tecnica.

## 5. Identidade soberana

- A identidade nao deve ficar presa ao app.
- A mesma pessoa deve poder operar por varios apps sem perder autonomia.
- O desenho mais consistente e uma raiz soberana com personas derivadas por contexto.
- O protocolo deve suportar rotacao, revogacao e prova de vinculacao entre personas quando necessario.
- O app deve ser cliente de identidade, nao dono da identidade.

## 6. Metadados e atributos

- Metadados pessoais nao devem ser obrigatorios para participar.
- Atributos uteis devem ser opcionais, minimizados e atualizaveis com seguranca.
- O sistema deve favorecer predicados e provas, nao exposicao bruta de dados.
- Exemplo: em vez de publicar idade, publicar prova de `18+` ou `21+` quando a comunidade pedir isso.
- Mudancas futuras nao devem apagar o historico de algo ja assinado, mas podem alterar autorizacoes futuras.

## 7. Cold wallets e soberania

- O sistema pode suportar cofres frios, carteiras quentes e dispositivos de alta seguranca.
- Isso e opcional, nunca fator de forma obrigatorio.
- Nao pode haver coleira vestivel como requisito de participacao.
- Nenhum dispositivo corporal deve virar condicao basica de existencia no sistema.

## 8. Separacao de camadas

- Protocolo: forma, causalidade, verificabilidade e interoperabilidade minima.
- Identidade: raiz, personas, commitments, provas e rotacao.
- Valor: regime local de emissao e reconhecimento.
- App: UX, politica local e automacoes.
- Capsula: semantica executavel de uma instituicao ou fluxo local.
- Federacao: compensacao e traducao limitada entre mundos diferentes.

## 9. Riscos a evitar

- Confundir nome do protocolo com nome da moeda.
- Fazer do app a definicao pratica da comunidade.
- Fazer da capsula uma ontologia total da vida social.
- Fazer do vault um dispositivo obrigatorio e coercitivo.
- Centralizar a borda e chamar isso de interoperabilidade.
- Transformar credenciais opcionais em cadastro universal.

## 10. Tarefas para a proxima etapa

- Fechar um glossario canônico para protocolo, node, Commoner, app, capsula, comunidade e valor.
- Revisar a spec para separar valor local de borda federativa.
- Revisar a arquitetura de identidade para raiz, personas e provas opcionais.
- Revisar o desenho de apps para evitar que UX vire norma ontologica.
- Revisar docs e exemplos para remover defaults que sugiram moeda universal.

## 11. Formula curta

- Comunidades governam valor.
- O protocolo governa forma minima.
- Apps governam experiencia.
- Capsulas governam semantica local executavel.
- Identidade deve ser soberana, portavel e nao obrigatoriamente vestivel.
