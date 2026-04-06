# Teoria do Protocolo

## Fundamento

O Comum Protocol se organiza a partir de um unico atomo: **Testimony**.
Tudo o que o sistema faz e uma composicao de testemunhos assinados e
verificaveis. Nao existe camada magica: apenas evidencia, contexto e
historia.

## Separacao essencial

- **Semantica** (Claim): o que esta sendo afirmado.
- **Justificativa** (Context + Proof): por que a afirmacao e confiavel.

Esta separacao impede que o protocolo confunda significado com prova.
Uma comunidade pode trocar o criterio de prova sem alterar a semantica.

## Camadas

1) **Narrativa** (humana): o sentido social do ato.
2) **Semantica** (Claim): verbo + payload do testemunho.
3) **Justificativa** (Context/Proof): assinatura, contexto, nulificacao.
4) **Persistencia** (Graph): referencias, continuidade e memoria coletiva.
5) **Transporte** (CTE/Sync): reconcilia grafos em ambiente offline-first.

## Abstracoes nucleares

- **Testimony**: evidencia minima, assinada e reproduzivel.
- **Claim**: descricao formal do ato (verbo + payload).
- **Context**: condicao de emissao (proximidade, beacon, place, vouch).
- **Proof**: conjunto de provas (assinaturas, zk, nullifiers).
- **Graph**: historia compartilhada derivada de referencias.
- **Flow**: sequencia de Testimonies que realiza um ato social completo.

## Papéis conceituais

- **Commoner**: agente social que emite/recebe Testimonies.
- **Witness**: a funcao de testemunhar (ato de assinar e provar).
- **Node**: entidade tecnica que armazena, valida e sincroniza o grafo.

Nenhum desses papeis e obrigatório na implementacao, mas ajudam a manter
linguagem ubiqua consistente.

## Regras de composicao

- Fluxos sao contratos sociais expressos em sequencias de Testimonies.
- Capsulas apenas automatizam regras sobre fluxos; nao criam novos atomos.
- A verdade operacional e local e verificavel; consenso global e opcional.

## Criterios de legitimidade

- **Local-first**: o que acontece localmente e valido sem rede.
- **Determinismo**: CBOR canonical e hashes estaveis garantem reproducao.
- **Auditabilidade**: qualquer historico pode ser reconstituido.
- **Privacidade por padrao**: minimiza correlacoes e vazamentos.

## Anti-padroes

- Centralizar prova ou validacao em uma entidade global.
- Misturar semantica com prova e congelar os criterios de legitimidade.
- Esconder limites do protocolo atras de UX ou infraestrutura.
