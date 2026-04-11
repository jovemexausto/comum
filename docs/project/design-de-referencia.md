# Princípios de design para implementações de referência do Comum

**Status:** rascunho de trabalho  
**Escopo:** implementações de referência produzidas pelo projeto Comum  
**Relação com o protocolo:** complementar, não substituta

Este documento reúne notas operacionais para implementadores que atuam no Comum.
Sua fundação teórica e política se encontra no Liivro do Comum (capítulo 13: "Design como ética da forma").

---

## Notas para implementadores

Para quem constrói sobre o protocolo:

- **1. A vida acontece fora do dispositivo**: Projete para sessões curtas. Uma sessão bem-sucedida é aquela que termina cedo porque o que precisava ser feito foi feito.
- **2. Silêncio como padrão**: As implementações de referência não puxam. Notificações devem vir desligadas de fábrica.
- **3. Favoreça o encontro**: Um modo de proximidade, ativado quando as pessoas estão fisicamente juntas via mesh local, serve a coordenação ao vivo. Não converta presença em dados além do necessário. O protocolo permite não registrar tudo.
- **4. Sem mecânicas de engajamento**: Não exiba quantas pessoas viram, quem respondeu mais rápido, nem contadores públicos de engajamento comunitário. O Comum não quer gerenciar competição.
- **5. Offline como normalidade**: Estar offline não é degradação, é repouso. Desenhe sem letreiros alarmistas vermelhos de "Sem conexão".  A sincronização é fofoca de fundo, acontece quando pode.
- **6. Saída explícita**: Explicite na arquitetura o que é história da comunidade, e facilite a expotação e migração desde o primeiro commit. A porta aberta é a única forma digna de retenção.

---

Para refletir durante o desenvolvimento (perguntas rápidas):

- Para cada notificação que você for implementar: a quem ela serve?
- Para cada contador público: o que ele faz com a cultura interna da comunidade?
- Qual a utilidade real desse atestado? Precisa ser global ou o scope local resolve?
