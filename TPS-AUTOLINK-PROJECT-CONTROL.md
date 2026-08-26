---
document: TPS-AUTOLINK-PROJECT-CONTROL
title: "TPS AutoLink — Controle Mestre de Projeto e Anti-Deriva LLM"
version: "1.0.0"
status: "CANONICAL"
authority: "USER_APPROVED_PROJECT_CONTROL"
project: "TPS AutoLink"
project_phase: "IMPLEMENTATION"
active_gate: "AUTO-01"
roadmap_first_gate: "AUTO-00"
roadmap_last_gate: "AUTO-18"
branch_budget: 0
default_author_language: "Rust"
production_release_target: "v1.0.0"
production_before_enhancement: true
last_updated: "2026-08-26"
---

# TPS AutoLink — Controle Mestre de Projeto e Anti-Deriva LLM

> **Documento canônico de controle do projeto.**
>
> Este arquivo existe para manter qualquer LLM, agente de engenharia, desenvolvedor ou sessão de trabalho dentro do escopo aprovado do TPS AutoLink até a conclusão, certificação e entrada em produção da versão 1.0.0.
>
> **Princípio supremo:** `FINISH > EXPAND`.
>
> Nenhuma melhoria, tecnologia interessante, refatoração opcional, nova integração, nova plataforma, nova versão, nova arquitetura ou nova ideia pode interromper a conclusão do roadmap canônico `AUTO-00 → AUTO-18`, salvo bloqueio técnico real, evidência de falha, alteração explícita de requisito ou autorização explícita do usuário.

---

# 0. COMO USAR ESTE DOCUMENTO

## 0.1 Finalidade

Este documento é simultaneamente:

1. constituição do projeto;
2. contrato operacional de engenharia;
3. máquina de estados;
4. controle anti-deriva para LLM;
5. registro de escopo;
6. registro de decisões congeladas;
7. registro de bloqueios;
8. registro de dívida técnica;
9. contrato de conclusão de cada gate;
10. critério de passagem para produção.

Ele **não** é um documento de brainstorming.

Ele **não** autoriza expansão automática de escopo.

Ele **não** deve ser reinterpretado para criar novos projetos.

## 0.2 Regra de precedência

Quando houver conflito entre instruções durante uma sessão, aplicar a seguinte ordem:

1. instrução explícita atual do usuário;
2. este documento canônico;
3. requisitos aprovados do gate ativo;
4. decisões congeladas registradas;
5. especificações técnicas aprovadas;
6. implementação existente aprovada;
7. documentação externa;
8. sugestões do LLM.

Uma sugestão do LLM nunca pode sobrepor uma decisão congelada sem autorização.

## 0.3 Como iniciar qualquer sessão

Ao receber este documento, o LLM deve começar identificando:

```text
PROJECT
ACTIVE_GATE
GATE_STATUS
GATE_OBJECTIVE
EXIT_CRITERIA
BLOCKERS
FROZEN_DECISIONS
NEXT_EXACT_ACTION
```

Antes de escrever código, propor arquitetura ou iniciar pesquisa, o LLM deve responder internamente:

```text
A ação solicitada é necessária para concluir o ACTIVE_GATE?
```

Se `SIM`, prosseguir.

Se `NÃO`, classificar como:

- `NON_BLOCKING`;
- `FUTURE_GATE`;
- `OUT_OF_SCOPE`.

e **não executar agora**, salvo autorização expressa.

## 0.4 Como encerrar qualquer sessão

Toda sessão técnica deve terminar com o bloco:

```text
PROJECT:
ACTIVE_GATE:
STATUS:
PROVEN:
PENDING:
BLOCKERS:
NON_BLOCKING:
FROZEN:
ARTIFACTS:
NEXT_EXACT_ACTION:
```

`NEXT_EXACT_ACTION` deve conter **uma única próxima ação canônica**, não uma lista de possibilidades.

---

# 1. MISSÃO DO PROJETO

Construir, testar, certificar fisicamente e colocar em produção o **TPS AutoLink**, uma plataforma própria de infotainment e conectividade automotiva, capaz de utilizar o telefone celular como fonte de conectividade WAN e de serviços pessoais, mantendo uma arquitetura própria, modular, segura, testável, versionada e independente de um único fabricante.

O produto deve poder evoluir posteriormente para integrações oficiais com plataformas automotivas de terceiros, sem fazer dessas integrações uma dependência do núcleo.

---

# 2. OBJETIVO DA VERSÃO 1.0.0

A versão 1.0.0 deve demonstrar, de forma compilável, executável, testada e fisicamente verificável:

1. descoberta entre telefone e head unit de referência;
2. pareamento seguro;
3. sessão autenticada;
4. conectividade de Internet fornecida pelo telefone;
5. comunicação bidirecional;
6. reprodução de áudio local;
7. reprodução de rádio/streaming via Internet;
8. comandos de mídia;
9. perda e recuperação de conexão;
10. cache local controlado;
11. handover de conectividade suportado;
12. interface touchscreen funcional;
13. integração Android Auto, quando tecnicamente permitida pelo escopo oficial;
14. integração Android Automotive, quando tecnicamente permitida;
15. integração com iPhone no escopo aprovado;
16. integração CarPlay/MFi somente pelo processo oficial, quando autorizada;
17. testes de segurança, carga, soak e resiliência;
18. teste veicular controlado e certificação da versão 1.0.0.

---

# 3. PRINCÍPIO DE ENTREGA

## 3.1 Produção antes de aprimoramento

Até `AUTO-18 = PASS` e a release `v1.0.0` estar declarada `PRODUCTION_READY`:

```text
NOVAS_FEATURES = PROIBIDAS
NOVAS_VERSOES = PROIBIDAS
ARQUITETURA_PARALELA = PROIBIDA
REESCRITA_OPCIONAL = PROIBIDA
OTIMIZACAO_NAO_EXIGIDA = PROIBIDA
```

Exceções:

```text
SECURITY_BLOCKER
FUNCTIONAL_BLOCKER
COMPLIANCE_BLOCKER
HARDWARE_BLOCKER
FAILED_TEST
USER_REQUIREMENT_CHANGE
USER_AUTHORIZATION
```

## 3.2 Depois de produção

Somente após:

```text
AUTO-18 = PASS
AND
RELEASE = v1.0.0
AND
PRODUCTION_READY = TRUE
```

podem ser abertas:

```text
v1.0.x = correções
v1.1.x = melhorias compatíveis
v2.x   = mudanças maiores
```

Nenhuma dessas linhas deve existir operacionalmente antes da produção da 1.0.0.

---

# 4. LIMITES DO PRODUTO

## 4.1 Escopo inicial

TPS AutoLink v1.0.0 é uma plataforma de:

- infotainment;
- mídia;
- conectividade;
- interface automotiva;
- áudio;
- streaming;
- cache;
- descoberta;
- autenticação;
- telemetria do próprio sistema;
- controle de sessão;
- integração com APIs de plataforma permitidas.

## 4.2 Fora do escopo v1.0.0

Sem projeto separado e autorização explícita, são proibidos:

- controle de direção;
- controle de freio;
- controle de aceleração;
- powertrain;
- airbag;
- ADAS safety-critical;
- imobilizador;
- ECU crítica;
- reprogramação de ECU;
- mensagens CAN que alterem funções safety-critical;
- bypass de mecanismos de segurança de fabricantes;
- emulação não autorizada de tecnologias licenciadas;
- quebra de DRM;
- contorno de proteção de plataformas;
- clonagem de identidade de head units certificadas.

A existência de uma interface técnica não equivale a autorização para utilizá-la.

---

# 5. ARQUITETURA CANÔNICA DE ALTO NÍVEL

```text
                      INTERNET
                         |
             +-----------+-----------+
             |                       |
         4G / 5G                  Wi-Fi WAN
             |                       |
             +-----------+-----------+
                         |
                    SMARTPHONE
                         |
            +------------+------------+
            |            |            |
           USB         Wi-Fi          BLE
            |            |            |
            +------------+------------+
                         |
                 TPS AUTOLINK PROTOCOL
                         |
                  TPS HEAD UNIT REF
                         |
            +------------+------------+
            |                         |
         DISPLAY                    AUDIO
```

## 5.1 Princípio de separação

O sistema deve separar:

```text
TRANSPORTE
PROTOCOLO
SESSAO
SEGURANCA
MIDIA
CACHE
TELEMETRIA
UI
INTEGRACOES
HARDWARE
```

Uma mudança de transporte não deve exigir reescrita da lógica central.

Uma mudança de UI não deve exigir reescrita do protocolo.

Uma integração de terceiro não deve se tornar dependência do núcleo.

---

# 6. POLÍTICA DE LINGUAGEM

## 6.1 Regra padrão

```text
CODIGO_AUTORAL_NOVO => RUST
```

## 6.2 Exceções válidas

Outra linguagem só pode ser usada quando houver justificativa objetiva, registrada como decisão:

- API nativa de plataforma;
- framework oficial;
- toolchain obrigatório;
- vendor SDK;
- ABI;
- target não suportado;
- requisito de certificação;
- UI nativa;
- restrição comprovada.

Exemplos aceitáveis:

```text
Swift       -> camada UI/API Apple
Kotlin      -> camada UI/API Android
TypeScript  -> frontend web, se houver
C           -> FFI/ABI/vendor/hardware específico
C++         -> integração obrigatória com framework C++
```

## 6.3 Rust-first não significa Rust-only

O objetivo não é maximizar porcentagem de Rust.

O objetivo é minimizar complexidade e superfície insegura mantendo compatibilidade.

---

# 7. POLÍTICA DE `unsafe`, C E FFI

## 7.1 Regra

Todo `unsafe` deve ser:

- mínimo;
- isolado;
- documentado;
- testado;
- justificado;
- encapsulado por API segura quando possível.

## 7.2 C

Código C autoral não deve ser criado por preferência cultural.

Só criar C quando existir uma necessidade objetiva.

## 7.3 FFI

Toda fronteira FFI deve definir explicitamente:

- ownership;
- lifetime;
- nullability;
- allocation;
- deallocation;
- thread-safety;
- error model;
- panic/unwind behavior;
- ABI version;
- struct layout;
- alignment;
- buffer ownership;
- string encoding;
- callback lifetime.

---

# 8. ROADMAP CANÔNICO

O roadmap abaixo é fechado para a versão 1.0.0.

```text
AUTO-00
  |
AUTO-01
  |
AUTO-02
  |
AUTO-03
  |
AUTO-04
  |
AUTO-05
  |
AUTO-06
  |
AUTO-07
  |
AUTO-08
  |
AUTO-09
  |
AUTO-10
  |
AUTO-11
  |
AUTO-12
  |
AUTO-13
  |
AUTO-14
  |
AUTO-15
  |
AUTO-16
  |
AUTO-17
  |
AUTO-18
  |
PRODUCTION v1.0.0
```

É proibido criar novos gates por iniciativa do LLM.

Subtarefas internas podem existir dentro de um gate, mas:

```text
SUBTASK != NEW_GATE
```

---

# 9. GATES CANÔNICOS

## AUTO-00 — Fundação do projeto

### Objetivo

Criar a base reproduzível do projeto.

### Deve entregar

- workspace Rust;
- estrutura de repositório;
- política de versões;
- CI;
- formatação;
- lint;
- testes básicos;
- simulador inicial;
- especificação inicial do TPS AutoLink Protocol v1;
- documentação de build;
- documentação de execução;
- controle de dependências;
- política de segurança;
- mecanismo de evidências;
- build reproduzível na plataforma de referência definida.

### Critério de saída

```text
cargo fmt --check = PASS
cargo check = PASS
cargo clippy = PASS
cargo test = PASS
release build = PASS
simulator starts = PASS
protocol v1 skeleton = FROZEN
repository structure = FROZEN
```

### Não pertence ao gate

- streaming real;
- Android Auto;
- CarPlay;
- cache completo;
- UI final;
- veículo físico.

---

## AUTO-01 — Descoberta Phone ↔ Head Unit

### Objetivo

Telefone e head unit de referência devem se descobrir de forma controlada.

### Deve entregar

- identidade de dispositivo;
- anúncio de presença;
- descoberta;
- timeout;
- retry;
- seleção de peer;
- logs;
- testes de descoberta positiva e negativa.

### Critério de saída

Telefone detecta a head unit aprovada e a head unit identifica o telefone esperado.

### Não pertence

- autenticação completa;
- streaming;
- cache;
- cloud;
- CarPlay.

---

## AUTO-02 — Pareamento criptográfico

### Objetivo

Estabelecer uma relação de confiança entre telefone e head unit.

### Deve entregar

- fluxo de pairing;
- identidade persistente;
- confirmação;
- proteção contra replay aplicável;
- revogação local;
- teste de peer inválido;
- teste de chave inválida;
- teste de repetição.

### Critério de saída

Dois dispositivos aprovados estabelecem confiança e peer não autorizado é rejeitado.

---

## AUTO-03 — Sessão autenticada

### Objetivo

Abrir, manter e encerrar sessão autenticada do TPS AutoLink.

### Deve entregar

- session state machine;
- session ID;
- autenticação;
- keepalive;
- timeout;
- close;
- reconnect básico;
- tratamento de versão incompatível.

### Critério de saída

Sessão autenticada abre, permanece íntegra, encerra e rejeita sessão inválida.

---

## AUTO-04 — Internet do telefone para a Head Unit

### Objetivo

A head unit obtém conectividade WAN funcional através do telefone no método aprovado.

### Deve entregar

- detecção de disponibilidade;
- estabelecimento do caminho de dados;
- teste DNS;
- teste TCP/QUIC/HTTP conforme necessário;
- detecção de perda;
- metering/roaming quando a plataforma fornecer;
- telemetria básica.

### Critério de saída

Head unit acessa endpoint de teste através da conectividade fornecida pelo telefone e registra métricas.

---

## AUTO-05 — Controle bidirecional

### Objetivo

Telefone e head unit trocam comandos e eventos de aplicação.

### Deve entregar

- mensagens command/event;
- request ID;
- acknowledgment;
- timeout;
- duplicate handling;
- invalid command handling.

### Critério de saída

Comandos aprovados percorrem as duas direções com confirmação e erros controlados.

---

## AUTO-06 — Áudio local

### Objetivo

Reproduzir áudio controlado na head unit sem depender ainda do serviço de rádio público.

### Deve entregar

- pipeline local;
- play;
- pause;
- stop;
- volume lógico;
- metadados básicos;
- teste de underrun;
- telemetria de startup.

### Critério de saída

Áudio local reproduz continuamente conforme critérios definidos sem crash e com métricas.

---

## AUTO-07 — Rádio/streaming Internet

### Objetivo

Reproduzir stream real através da conectividade fornecida pelo telefone.

### Deve entregar

- source URL controlada;
- connect;
- buffering;
- playback;
- error handling;
- reconnection;
- metadata quando disponível;
- medição de startup.

### Critério de saída

Stream real inicia, reproduz, reconecta e gera evidência mensurável.

---

## AUTO-08 — Controles de mídia

### Objetivo

Completar controles necessários de uso automotivo.

### Deve entregar

- play;
- pause;
- previous/next quando aplicável;
- favorites;
- source select;
- volume control dentro da camada permitida;
- metadata display;
- command debouncing.

### Critério de saída

Todos os controles v1 aprovados operam e têm teste automatizado ou evidência física.

---

## AUTO-09 — Perda e recuperação de conexão

### Objetivo

O sistema deve sobreviver a interrupções previsíveis.

### Deve entregar

- loss detection;
- reconnect policy;
- backoff;
- state restoration;
- duplicate suppression;
- session recovery quando aprovado;
- métricas de gap.

### Critério de saída

Cenários aprovados de interrupção não causam crash e recuperam dentro dos limites definidos.

---

## AUTO-10 — Cache

### Objetivo

Criar cache local controlado para continuidade de mídia e redução de dependência da rede.

### Deve entregar

- cache key;
- integrity;
- TTL;
- size limit;
- eviction;
- hit/miss;
- corruption handling;
- live-content policy;
- storage policy.

### Critério de saída

Cache funciona nos cenários aprovados sem mascarar incorretamente o estado de conteúdo ao vivo.

---

## AUTO-11 — Handover de rede

### Objetivo

Suportar mudança entre caminhos de conectividade aprovados sem quebrar a aplicação.

### Deve entregar

- network observer;
- path quality;
- transition handling;
- stream recovery;
- metrics;
- tests with simulated degradation.

### Critério de saída

Mudanças suportadas de conectividade recuperam ou mantêm a sessão dentro dos limites definidos.

---

## AUTO-12 — Interface touchscreen

### Objetivo

Entregar UI funcional de referência na head unit.

### Deve entregar

- home;
- media selection;
- now playing;
- status;
- connection status;
- safe interaction model;
- parked/moving policy quando houver sinal de estado confiável;
- accessibility básica definida.

### Critério de saída

Fluxo principal é utilizável na tela de referência e não interfere nos gates anteriores.

---

## AUTO-13 — Android Auto

### Objetivo

Integrar o produto ao Android Auto exclusivamente dentro das APIs e categorias oficialmente permitidas.

### Regra

Se uma função desejada não for permitida pela plataforma, ela não será emulada por meios não autorizados.

### Critério de saída

Integração aceita no escopo técnico oficial selecionado e testada em ambiente compatível.

---

## AUTO-14 — Android Automotive OS

### Objetivo

Executar/integrar o TPS AutoLink em ambiente AAOS compatível.

### Critério de saída

Aplicação executa e oferece o subconjunto aprovado de funções em ambiente AAOS de referência.

---

## AUTO-15 — iPhone

### Objetivo

Entregar integração iPhone para o TPS AutoLink próprio.

### Deve entregar

- camada nativa necessária;
- TPS Rust Core reutilizado quando aplicável;
- pairing/session;
- connectivity path permitido;
- media control permitido.

### Critério de saída

iPhone se comunica com a head unit pelo TPS AutoLink próprio no escopo aprovado.

---

## AUTO-16 — CarPlay / MFi

### Objetivo

Executar somente integrações oficialmente permitidas/licenciadas.

### Regra de bloqueio

Se credencial, licença, hardware, entitlement ou aprovação oficial for necessária e não estiver disponível:

```text
AUTO-16 = EXTERNAL_DEPENDENCY_BLOCKED
```

Isso **não autoriza emulação não licenciada**.

O usuário decide se:

- obtém a dependência;
- redefine formalmente o critério de release;
- adia o gate para versão futura.

O LLM não toma essa decisão sozinho.

---

## AUTO-17 — Segurança, carga, soak e resiliência

### Objetivo

Certificar tecnicamente a plataforma antes do teste veicular final.

### Deve incluir, conforme aplicável

- unit tests;
- integration tests;
- protocol tests;
- fuzzing;
- malformed frames;
- replay attempts;
- authentication failures;
- reconnect storms;
- network loss;
- latency;
- jitter;
- packet loss;
- resource limits;
- memory use;
- CPU;
- long-running soak;
- log integrity;
- crash recovery;
- dependency audit;
- unsafe/FFI review;
- release build.

### Critério de saída

Nenhum blocker aberto e requisitos de qualidade aprovados atendidos.

---

## AUTO-18 — Teste veicular controlado e produção

### Objetivo

Validar fisicamente o TPS AutoLink no ambiente automotivo autorizado e congelar a release v1.0.0.

### Deve incluir

- checklist pré-teste;
- hardware identificado;
- software versionado;
- hashes;
- condições do teste;
- conexão;
- áudio;
- controles;
- reconnect;
- cache;
- handover aplicável;
- temperatura;
- CPU/RAM;
- logs;
- incidentes;
- rollback;
- resultado;
- as-built.

### Critério de saída

```text
ALL_REQUIRED_GATES = PASS
NO_OPEN_BLOCKER = TRUE
PHYSICAL_TEST = PASS
SECURITY_GATE = PASS
AS_BUILT_COMPLETE = TRUE
RELEASE_ARTIFACTS_HASHED = TRUE
v1.0.0 = FROZEN
PRODUCTION_READY = TRUE
```

---

# 10. MÁQUINA DE ESTADOS

Cada gate só pode estar em um destes estados:

```text
NOT_STARTED
READY
IN_PROGRESS
BLOCKED
FAILED
PASS
FROZEN
```

## 10.1 Transições permitidas

```text
NOT_STARTED -> READY
READY -> IN_PROGRESS
IN_PROGRESS -> BLOCKED
IN_PROGRESS -> FAILED
IN_PROGRESS -> PASS
BLOCKED -> IN_PROGRESS
FAILED -> IN_PROGRESS
PASS -> FROZEN
FROZEN -> somente reabertura formal autorizada
```

## 10.2 Transições proibidas

```text
NOT_STARTED -> PASS
BLOCKED -> PASS sem evidência
FAILED -> FROZEN
FROZEN -> IN_PROGRESS por iniciativa do LLM
```

---

# 11. UM ÚNICO GATE ATIVO

Em qualquer instante:

```text
COUNT(ACTIVE_GATE) = 1
```

Nunca dois.

Uma subtarefa de outro gate não se torna ativa porque parece conveniente.

## 11.1 Regra de pergunta obrigatória

Para toda nova ação:

```text
DOES_THIS_DIRECTLY_ADVANCE_ACTIVE_GATE?
```

Resultados:

```text
YES -> EXECUTE
NO  -> CLASSIFY
```

---

# 12. CLASSIFICAÇÃO OBRIGATÓRIA DE QUALQUER DESCOBERTA

Toda lacuna, bug, ideia, dependência, oportunidade ou problema novo deve entrar exatamente em uma categoria.

## A. BLOCKER

Definição:

Impede objetivamente o critério de saída do gate ativo.

Ação:

```text
RESOLVE_NOW
```

## B. NON_BLOCKING

Definição:

Importante, mas não impede o gate.

Ação:

```text
REGISTER
CONTINUE_ACTIVE_GATE
```

## C. FUTURE_GATE

Definição:

Pertence claramente a gate posterior.

Ação:

```text
REGISTER_IN_TARGET_GATE
DO_NOT_EXECUTE_NOW
```

## D. OUT_OF_SCOPE

Definição:

Não pertence à versão 1.0.0 ou ao produto aprovado.

Ação:

```text
IGNORE_FOR_V1
```

É proibido inventar categoria intermediária para justificar ramificação.

---

# 13. BRANCH BUDGET

```text
BRANCH_BUDGET = 0
```

Significa:

Não manter arquiteturas alternativas simultaneamente.

## 13.1 Exceção controlada

Uma análise de alternativas só pode ser aberta quando:

1. decisão é necessária para o gate ativo;
2. alternativas são mutuamente exclusivas;
3. evidência atual não permite escolha direta.

Nesse caso:

```text
COMPARE
MEASURE
DECIDE
FREEZE
DELETE_ALTERNATIVE_PATH
CONTINUE
```

Não manter:

```text
Option A em produção
Option B "para talvez"
Option C experimental
```

sem autorização.

---

# 14. REGRA DE DECISÃO CONGELADA

Uma decisão marcada `FROZEN` não pode ser reaberta apenas porque apareceu uma solução mais interessante.

Pode ser reaberta somente com:

```text
FAILURE_EVIDENCE
SECURITY_EVIDENCE
COMPLIANCE_EVIDENCE
REQUIREMENT_CHANGE
USER_AUTHORIZATION
```

## 14.1 Argumentos inválidos para reabertura

- “há uma biblioteca mais moderna”;
- “vi uma arquitetura melhor”;
- “outra empresa faz diferente”;
- “poderíamos aproveitar para...”;
- “talvez escale melhor no futuro”;
- “é elegante”;
- “é tendência”.

---

# 15. DECISÕES CONGELADAS INICIAIS

As seguintes decisões estão congeladas para início do projeto:

```text
FD-001: TPS AutoLink é produto próprio, não clone de CarPlay.
FD-002: núcleo autoral novo é Rust-first.
FD-003: Rust-first não significa Rust-only.
FD-004: smartphone fornece WAN/conectividade na arquitetura inicial.
FD-005: head unit Linux de referência é o alvo inicial de bancada.
FD-006: BLE pode ser usado para descoberta/controle leve quando apropriado.
FD-007: canal de dados principal deve usar transporte adequado a volume/latência, não depender de BLE.
FD-008: integrações Android Auto/AAOS/Apple são adaptadores, não núcleo.
FD-009: CarPlay/MFi só por caminho oficial/licenciado.
FD-010: v1.0.0 é infotainment; funções safety-critical de veículo ficam fora.
FD-011: um gate ativo por vez.
FD-012: roadmap AUTO-00→AUTO-18 é canônico.
FD-013: nenhum novo gate sem autorização explícita do usuário.
FD-014: produção v1.0.0 antecede aprimoramentos e novas versões.
FD-015: dívida técnica não bloqueante não interrompe gate.
FD-016: sem evidência suficiente, estado = UNKNOWN/NOT_PROVEN.
FD-017: código final deve ser testado antes de ser apresentado como produção.
FD-018: FFI/unsafe devem permanecer confinados e auditáveis.
```

---

# 16. POLÍTICA DE LACUNAS E `IF/ELSE`

Projetos grandes contêm milhares de condições.

O objetivo não é eliminar IF/ELSE.

O objetivo é impedir que IF/ELSE criem novos projetos.

## 16.1 Regra

Ao descobrir:

```text
IF condição X
ELSE condição Y
```

o LLM deve perguntar:

```text
AMBOS precisam ser resolvidos agora para aprovar o gate?
```

Se não:

- resolver apenas o ramo atual necessário;
- registrar o outro ramo;
- continuar.

## 16.2 Proibição

É proibido transformar cada combinação de:

```text
OS
device
network
vendor
codec
car
radio
screen
transport
```

em um subprojeto antes que haja necessidade comprovada.

---

# 17. DEBT LEDGER

Dívida técnica existe para impedir que melhorias opcionais interrompam o roadmap.

Formato:

```text
DEBT-ID:
DISCOVERED_IN:
DESCRIPTION:
CLASSIFICATION:
WHY_NOT_BLOCKING:
TARGET_VERSION:
STATUS:
```

Exemplo:

```text
DEBT-001
DISCOVERED_IN: AUTO-01
DESCRIPTION: avaliar UWB para descoberta de proximidade
CLASSIFICATION: FUTURE
WHY_NOT_BLOCKING: BLE discovery atende o critério v1
TARGET_VERSION: POST-v1
STATUS: DEFERRED
```

## 17.1 Regra

```text
DEBT != BLOCKER
```

Somente evidência objetiva pode promover uma dívida para blocker.

---

# 18. POLÍTICA DE PESQUISA

Pesquisa só é permitida quando necessária para:

- verificar fato material;
- resolver blocker;
- confirmar API/standard;
- escolher dependência do gate ativo;
- validar requisito de plataforma;
- confirmar segurança/compliance.

Pesquisa não deve virar exploração aberta.

## 18.1 Stop condition

Pesquisa termina quando houver evidência suficiente para tomar a decisão necessária do gate.

Não continuar “para ver se existe algo ainda melhor”.

---

# 19. POLÍTICA DE CÓDIGO

Nenhum código deve ser apresentado como final sem validação compatível com sua função.

## 19.1 Mínimo Rust

Quando aplicável:

```text
cargo fmt --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets
cargo build --workspace --release
```

A linha exata pode ser adaptada quando uma feature/target impossibilitar o comando global, mas a exceção deve ser documentada.

## 19.2 Testes adicionais

Conforme o módulo:

- unit tests;
- integration tests;
- negative tests;
- malformed input;
- fuzzing;
- property tests;
- concurrency tests;
- reconnect;
- timeout;
- resource exhaustion;
- compatibility;
- migration;
- rollback.

## 19.3 Proibição de “tentativa”

Não usar linguagem como:

```text
"deve funcionar"
"provavelmente"
"tente este código"
```

quando o artefato é apresentado como produção.

Se algo não foi executado/testado:

```text
NOT_EXECUTED
NOT_PROVEN
```

---

# 20. POLÍTICA DE EVIDÊNCIA

Toda afirmação de `PASS` deve ter evidência.

Tipos aceitos:

```text
BUILD_LOG
TEST_LOG
HASH
BENCHMARK
PACKET_CAPTURE
METRIC
SCREENSHOT
DEVICE_LOG
PHYSICAL_TEST
OFFICIAL_SPEC
REPRODUCIBLE_COMMAND
```

## 20.1 Evidência mínima de release

Cada artefato de release deve possuir:

- nome;
- versão;
- commit;
- build target;
- build command;
- timestamp;
- SHA-256;
- testes vinculados;
- resultado.

---

# 21. POLÍTICA DE BLOQUEIO

Um blocker precisa responder:

```text
WHAT:
WHY:
EVIDENCE:
WHICH_EXIT_CRITERION_FAILS:
MINIMUM_FIX:
HOW_TO_VERIFY:
```

Sem isso, não pode ser usado para expandir escopo.

## 21.1 Bloqueio externo

Dependência externa como:

- licença;
- conta;
- certificado;
- entitlement;
- hardware;
- API do fabricante;

deve ser marcada:

```text
EXTERNAL_DEPENDENCY
```

O LLM não deve substituí-la por workaround não autorizado.

---

# 22. POLÍTICA DE SEGURANÇA

## 22.1 Segurança por padrão

- autenticação antes de operações privilegiadas;
- trust explícito;
- least privilege;
- segredos fora do código;
- chaves não logadas;
- inputs não confiáveis validados;
- versões de protocolo verificadas;
- replay considerado;
- mensagens malformadas rejeitadas;
- logs sem segredos;
- dependências auditadas;
- updates assinados quando este mecanismo entrar no escopo.

## 22.2 Criptografia

Não inventar algoritmo criptográfico próprio.

Primitivas devem vir de implementações consolidadas e apropriadas ao caso.

---

# 23. POLÍTICA DE TELEMETRIA

Toda otimização deve ser guiada por medição.

Métricas possíveis:

```text
startup_ms
rtt_ms
jitter_ms
packet_loss_pct
reconnect_ms
handover_ms
audio_gap_ms
buffer_ms
cache_hit_pct
cache_miss_pct
throughput
cpu_pct
ram_bytes
temperature
battery_impact
session_failures
```

Não transformar percepção subjetiva em fato técnico.

---

# 24. POLÍTICA DE PERFORMANCE

Nenhuma otimização prematura.

Processo:

```text
REQUIREMENT
MEASURE
IDENTIFY BOTTLENECK
OPTIMIZE
REMEASURE
```

Não reescrever módulo por hipótese de performance.

---

# 25. POLÍTICA DE DEPENDÊNCIAS

Adicionar dependência somente se:

1. resolve requisito do gate;
2. manutenção é aceitável;
3. licença é compatível;
4. segurança é aceitável;
5. target é suportado;
6. não cria complexidade desnecessária.

Toda nova dependência deve ser registrada.

---

# 26. POLÍTICA DE COMPATIBILIDADE

A v1.0.0 não precisa suportar todos os carros, rádios ou telefones do mundo.

Primeiro:

```text
REFERENCE_PHONE
+
REFERENCE_HEAD_UNIT
+
REFERENCE_OS
```

Depois expandir.

Compatibilidade universal antecipada é considerada expansão de escopo.

---

# 27. MATRIZ DE HARDWARE DE REFERÊNCIA

Antes do AUTO-01, registrar:

```text
PHONE_MODEL:
PHONE_OS:
HEAD_UNIT_MODEL:
HEAD_UNIT_OS:
CPU_ARCH:
RAM:
STORAGE:
DISPLAY:
AUDIO:
WIFI:
BLUETOOTH:
USB:
POWER:
```

Não assumir capacidades não comprovadas.

---

# 28. POLÍTICA DE SIMULADOR

Sempre que possível, criar simuladores para testar sem hardware.

O simulador não substitui teste físico.

Resultados devem distinguir:

```text
SIMULATED_PASS
PHYSICAL_PASS
```

Nunca declarar `PHYSICAL_PASS` com base em simulação.

---

# 29. POLÍTICA DE TESTE FÍSICO

Teste físico deve registrar:

- hardware exato;
- firmware/OS;
- software version;
- commit;
- hashes;
- topologia;
- rede;
- passos;
- resultados;
- logs;
- falhas;
- condições ambientais relevantes.

---

# 30. PROTOCOLO DE RESPOSTA DO LLM

## 30.1 Antes de agir

O LLM deve verificar:

```text
1. Qual é o ACTIVE_GATE?
2. Qual o EXIT_CRITERIA?
3. O pedido atual pertence ao gate?
4. Há blocker?
5. Estou prestes a criar nova ramificação?
6. Estou reabrindo FROZEN?
7. Estou resolvendo problema futuro?
8. Há uma menor ação suficiente?
```

## 30.2 Se houver ambiguidade

Se for possível executar com segurança uma escolha canônica já definida:

```text
USE_CANONICAL_DECISION
```

Se faltar informação não bloqueante:

```text
MARK_UNKNOWN
CONTINUE
```

Se faltar informação que impede o gate:

```text
BLOCKER
```

Não inventar.

## 30.3 Formato de abertura

Para tarefas técnicas relevantes:

```text
PROJECT: TPS AutoLink
ACTIVE_GATE: AUTO-XX
TARGET: <objetivo do gate>
SCOPE: AUTO-XX ONLY
BLOCKERS: <n>
```

## 30.4 Formato de encerramento

```text
PROJECT:
ACTIVE_GATE:
STATUS:
PROVEN:
PENDING:
BLOCKERS:
NON_BLOCKING:
FROZEN:
ARTIFACTS:
NEXT_EXACT_ACTION:
```

---

# 31. PROIBIÇÕES ESPECÍFICAS AO LLM

É proibido ao LLM:

1. criar novo gate sem autorização;
2. alterar roadmap por iniciativa própria;
3. iniciar implementação de gate futuro;
4. reabrir decisão congelada sem motivo válido;
5. criar arquitetura paralela para “comparar depois”;
6. multiplicar versões;
7. antecipar v2;
8. transformar melhoria em blocker;
9. afirmar teste físico que não ocorreu;
10. afirmar build/test que não executou;
11. esconder `UNKNOWN`;
12. preencher lacuna técnica por invenção;
13. usar “boas práticas” genéricas contra requisito aprovado sem evidência;
14. substituir produto próprio por plataforma de terceiro;
15. fazer reverse engineering não autorizado para contornar licenciamento;
16. expandir para sistemas safety-critical;
17. gerar grandes refatorações sem necessidade do gate;
18. propor mudança de linguagem sem evidência;
19. iniciar discussão de contratação/equipe quando não for requisito do gate;
20. continuar pesquisa após decisão suficiente;
21. adicionar tecnologia porque é moderna;
22. adicionar redundância sem requisito;
23. otimizar sem medição;
24. confundir dívida técnica com blocker;
25. confundir simulação com certificação física;
26. declarar produção antes do AUTO-18.

---

# 32. REGRA DE MENOR MUDANÇA SUFICIENTE

Quando existir falha:

```text
IDENTIFY_ROOT_CAUSE
APPLY_MINIMUM_CORRECT_FIX
TEST
CERTIFY
CONTINUE
```

Não usar uma falha pequena como justificativa para reconstruir módulos adjacentes.

Clean-room/rewrite só se a evidência mostrar que a arquitetura aprovada não consegue cumprir requisito ou se o usuário autorizar.

---

# 33. REGRA DE CONCLUSÃO

Quando o critério de saída do gate estiver atendido:

```text
PASS
FREEZE
NEXT_GATE
```

É proibido:

```text
PASS
BUT_WHILE_WE_ARE_HERE...
```

---

# 34. DEFINIÇÃO DE DONE

Um item está `DONE` somente se:

```text
IMPLEMENTED
+
TESTED
+
EVIDENCED
+
DOCUMENTED
+
EXIT_CRITERION_MET
```

Código escrito não equivale a `DONE`.

---

# 35. DEFINIÇÃO DE PRODUCTION READY

```text
PRODUCTION_READY =
    AUTO-00 PASS
AND AUTO-01 PASS
AND AUTO-02 PASS
AND AUTO-03 PASS
AND AUTO-04 PASS
AND AUTO-05 PASS
AND AUTO-06 PASS
AND AUTO-07 PASS
AND AUTO-08 PASS
AND AUTO-09 PASS
AND AUTO-10 PASS
AND AUTO-11 PASS
AND AUTO-12 PASS
AND REQUIRED_PLATFORM_GATES_RESOLVED
AND AUTO-17 PASS
AND AUTO-18 PASS
AND NO_BLOCKERS
AND RELEASE_FROZEN
```

A forma de tratar gates externos opcionais/condicionais, como integrações que dependam de licença, deve ser decidida formalmente pelo usuário antes da release. O LLM não os remove sozinho.

---

# 36. CHANGE CONTROL

Qualquer alteração em:

- missão;
- roadmap;
- gate;
- arquitetura canônica;
- linguagem padrão;
- limites de segurança;
- critério de produção;

deve gerar um registro:

```text
CHANGE-ID:
DATE:
REQUESTED_BY:
OLD:
NEW:
REASON:
IMPACT:
APPROVED:
```

Sem `APPROVED = USER`, a mudança não entra em vigor.

---

# 37. REGISTRO DE DECISÕES

Formato:

```text
DECISION-ID:
DATE:
GATE:
QUESTION:
OPTIONS:
EVIDENCE:
DECISION:
REASON:
STATUS: FROZEN
REOPEN_CONDITIONS:
```

---

# 38. REGISTRO DE BLOCKERS

Formato:

```text
BLOCKER-ID:
GATE:
DESCRIPTION:
EVIDENCE:
EXIT_CRITERION_IMPACT:
MINIMUM_FIX:
VERIFICATION:
OWNER:
STATUS:
```

---

# 39. REGISTRO DE ARTEFATOS

Formato:

```text
ARTIFACT-ID:
GATE:
NAME:
VERSION:
COMMIT:
SHA256:
TARGET:
BUILD_STATUS:
TEST_STATUS:
CERTIFICATION_STATUS:
```

---

# 40. ESTADO CANÔNICO ATUAL

> Esta seção é mutável. O restante do documento deve ser tratado como estável salvo Change Control.

```yaml
PROJECT: TPS-AutoLink
CONTROL_DOCUMENT_VERSION: 1.0.0

PHASE: IMPLEMENTATION

ACTIVE_GATE: AUTO-01
GATE_STATUS: READY

GATE_OBJECTIVE:
  Establish controlled discovery between the approved reference phone
  and the approved reference head unit.

BLOCKERS: []

NON_BLOCKING: []

FUTURE_GATE_BACKLOG: []

OUT_OF_SCOPE: []

FROZEN_DECISIONS:
  - FD-001
  - FD-002
  - FD-003
  - FD-004
  - FD-005
  - FD-006
  - FD-007
  - FD-008
  - FD-009
  - FD-010
  - FD-011
  - FD-012
  - FD-013
  - FD-014
  - FD-015
  - FD-016
  - FD-017
  - FD-018

CURRENT_ARTIFACTS:
  - TPS-AUTOLINK-PROJECT-CONTROL.md
  - Cargo.toml
  - rust-toolchain.toml
  - PROTOCOL.md
  - SECURITY.md
  - .github/workflows/auto-00-ci.yml
  - crates/tps-auto-protocol
  - crates/tps-auto-core
  - tools/tps-auto-simulator
  - docs/AUTO-00-EVIDENCE.md
  - docs/AUTO-00-SOURCES.md
  - docs/decisions/AUTO-00.md

NEXT_EXACT_ACTION:
  Register the approved reference phone/head-unit hardware matrix required before AUTO-01 implementation.

PRODUCTION_READY: false
```

---

# 41. CHECKSUM E DISCIPLINA DO DOCUMENTO

O repositório deve tratar este arquivo como fonte de verdade.

Quando alterado:

1. revisar diff;
2. registrar Change Control;
3. versionar;
4. commit;
5. opcionalmente registrar SHA-256 da versão usada na sessão.

Cabeçalho de sessão recomendado:

```text
CONTROL_FILE: TPS-AUTOLINK-PROJECT-CONTROL.md
CONTROL_VERSION: 1.0.0
CONTROL_COMMIT: <git commit>
CONTROL_SHA256: <sha256>
ACTIVE_GATE: AUTO-XX
```

Isso evita que duas sessões usem constituições diferentes sem perceber.

---

# 42. INTEGRAÇÃO COM REPOSITÓRIO

A fonte canônica deve permanecer no repositório.

Estrutura recomendada:

```text
tps-autolink/
├── TPS-AUTOLINK-PROJECT-CONTROL.md
├── Cargo.toml
├── README.md
├── SECURITY.md
├── PROTOCOL.md
├── crates/
├── phone/
├── headunit/
├── integrations/
├── tools/
├── tests/
└── docs/
```

Não criar cópias divergentes do arquivo em múltiplas pastas.

Se um resumo for necessário, ele deve apontar para este arquivo, não substituí-lo.

---

# 43. POLÍTICA DE USO COM CHATGPT/LLM

Em uma nova conversa, fornecer este arquivo ou disponibilizá-lo por integração com o repositório.

A instrução mínima deve ser:

```text
Leia TPS-AUTOLINK-PROJECT-CONTROL.md integralmente.
Ele é a autoridade operacional desta sessão.
Não altere roadmap, gate ativo, frozen decisions ou escopo sem minha autorização.
Execute somente o gate ativo.
Ao encontrar uma nova questão, classifique como BLOCKER, NON_BLOCKING,
FUTURE_GATE ou OUT_OF_SCOPE.
FINISH > EXPAND.
```

## 43.1 Se o LLM não conseguir acessar o arquivo

Estado:

```text
CONTROL_CONTEXT = UNAVAILABLE
```

Para mudanças arquiteturais, não prosseguir por memória aproximada.

Solicitar/acessar a versão canônica antes de alterar decisões.

Para trabalho local claramente delimitado que não altera governança, pode continuar somente se o usuário fornecer o contexto mínimo necessário.

---

# 44. TESTE ANTI-DERIVA DO LLM

Antes de aceitar qualquer proposta do LLM, aplicar:

```text
Q1: isso conclui o gate atual?
Q2: é blocker real?
Q3: já existe decisão congelada?
Q4: pertence a gate futuro?
Q5: está criando nova arquitetura?
Q6: foi medido ou é hipótese?
Q7: foi pedido pelo usuário?
Q8: podemos registrar no backlog e continuar?
```

Se `Q8 = SIM`, registrar e continuar.

---

# 45. EXEMPLOS DE COMPORTAMENTO CORRETO

## Exemplo A — UWB aparece durante AUTO-01

```text
DISCOVERY: UWB poderia melhorar presença.
ACTIVE_GATE_NEEDS_IT: NO
CLASSIFICATION: FUTURE_GATE / POST-v1
ACTION: REGISTER ONLY
```

## Exemplo B — BLE falha no hardware de referência

```text
DISCOVERY: hardware não suporta BLE funcional.
ACTIVE_GATE_NEEDS_DISCOVERY: YES
CLASSIFICATION: BLOCKER
ACTION: resolve minimum discovery path for AUTO-01
```

## Exemplo C — Biblioteca QUIC alternativa

Se a biblioteca atual atende requisitos:

```text
CLASSIFICATION: NON_BLOCKING
ACTION: do not migrate
```

## Exemplo D — Vulnerabilidade crítica na dependência

```text
CLASSIFICATION: BLOCKER
ACTION: remediate and recertify affected gate
```

---

# 46. EXEMPLOS DE COMPORTAMENTO INCORRETO

Errado:

```text
"Enquanto implementamos descoberta, vamos também preparar UWB,
CarPlay, Android Automotive e satélite."
```

Errado:

```text
"Rust está difícil aqui; vamos reescrever tudo em C++."
```

Errado:

```text
"Encontramos uma forma mais elegante, então vou reabrir três gates."
```

Errado:

```text
"Isso provavelmente passa."
```

Correto:

```text
NOT_PROVEN
```

---

# 47. REGRA DE RECUPERAÇÃO DE CONTEXTO

Se uma sessão longa perder contexto:

1. reler este arquivo;
2. reler estado do gate;
3. reler somente os artefatos do gate ativo;
4. reler decisões relacionadas;
5. continuar de `NEXT_EXACT_ACTION`.

Não reler todo o histórico do projeto se não for necessário.

---

# 48. REGRA DE COMPACTAÇÃO DE CONTEXTO

Para reduzir deriva, o contexto operacional preferido é:

```text
CONTROL DOCUMENT
+
ACTIVE GATE SPEC
+
CURRENT ARTIFACTS
+
LATEST TEST EVIDENCE
```

Não usar milhares de mensagens antigas como fonte primária quando o repositório contém o estado canônico.

---

# 49. REGRA DE AUTORIDADE DO REPOSITÓRIO

Quando o repositório conectado e este documento divergirem de memória conversacional:

```text
REPOSITORY_CANONICAL_STATE > CONVERSATIONAL_MEMORY
```

salvo instrução explícita atual do usuário.

---

# 50. REGRA DE NÃO-INVENÇÃO

Qualquer dado ausente deve ser:

```text
UNKNOWN
NOT_PROVEN
NOT_APPLICABLE
```

conforme o caso.

Nunca inferir:

- suporte de hardware;
- API;
- licença;
- desempenho;
- compatibilidade;
- estado de teste;
- segurança;
- certificação;
- disponibilidade de feature.

---

# 51. REGRA DE SAÍDA RÁPIDA

Se o gate pode ser concluído com uma solução menor que satisfaz integralmente o requisito:

```text
USE_MINIMUM_COMPLETE_SOLUTION
```

Não escolher uma solução maior apenas por potencial futuro.

---

# 52. REGRA DE ESCALABILIDADE

Escalabilidade só bloqueia a v1 se houver requisito quantificado da v1.

Futuro hipotético não pode bloquear release.

---

# 53. REGRA DE PORTABILIDADE

Portabilidade só é implementada para targets aprovados da v1.

Arquitetura deve evitar lock-in desnecessário, mas não precisa implementar todos os targets antecipadamente.

---

# 54. REGRA DE OBSERVABILIDADE

Cada gate adiciona apenas a observabilidade necessária para:

- provar seu funcionamento;
- diagnosticar sua falha;
- suportar operação segura.

Construção de plataforma completa de observabilidade não pode desviar o gate.

---

# 55. REGRA DE DOCUMENTAÇÃO

Documentação acompanha implementação.

Para cada componente final:

- purpose;
- inputs;
- outputs;
- errors;
- security assumptions;
- build;
- test;
- runtime;
- limits.

Não escrever documentação extensa de módulos que ainda não existem, salvo especificação necessária ao gate.

---

# 56. REGRA DE API E PROTOCOLO

Mudanças incompatíveis devem ser explicitamente versionadas.

Nunca alterar silenciosamente formato de mensagem já congelado.

Compatibilidade deve ser testada quando exigida.

---

# 57. REGRA DE RELEASE

Release candidata deve ser gerada a partir de commit identificável.

Formato recomendado:

```text
v1.0.0-rc.1
v1.0.0-rc.2
...
v1.0.0
```

Release final só após AUTO-18.

---

# 58. REGRA DE ROLLBACK

Mudanças de runtime/hardware capazes de impedir teste devem possuir caminho de retorno quando aplicável.

Rollback não precisa ser complexo quando o artefato é substituível por reinstalação reproduzível; nesse caso, documentar reinstalação como recuperação.

---

# 59. REGRA DE SEGREDOS

Nunca armazenar em Git:

- private keys;
- API tokens;
- passwords;
- signing secrets;
- certificates with private material;
- production credentials.

Documentar nomes e localização esperada, não valores secretos.

---

# 60. REGRA DE LOGS

Logs devem:

- permitir diagnóstico;
- evitar segredos;
- possuir timestamp;
- indicar módulo;
- indicar severidade;
- permitir correlação de sessão quando necessário.

---

# 61. REGRA DE ERROS

Erros devem ser:

- tipados quando adequado;
- propagados explicitamente;
- não ignorados silenciosamente;
- convertidos em métricas/logs quando necessário.

---

# 62. REGRA DE PANIC/CRASH

Componentes de runtime não devem depender de panic como fluxo normal.

Panic deve representar invariantes quebradas ou condições realmente excepcionais, conforme política do módulo.

---

# 63. REGRA DE CONCORRÊNCIA

Não adicionar concorrência sem necessidade.

Quando usada:

- ownership claro;
- cancelamento;
- timeout;
- shutdown;
- backpressure;
- race testing quando aplicável.

---

# 64. REGRA DE REDE

Protocolos devem considerar:

- perda;
- duplicação;
- reordenação;
- atraso;
- desconexão;
- reconexão;
- versão;
- tamanho máximo;
- input malformado.

---

# 65. REGRA DE CACHE

Cache nunca é fonte autoritativa quando a semântica exigir origem atual.

Cache deve possuir política explícita de validade.

---

# 66. REGRA DE DADOS PESSOAIS

Coletar apenas o necessário para o funcionamento aprovado.

Telemetria de desenvolvimento deve evitar dados pessoais desnecessários.

Qualquer expansão de coleta deve ser deliberada e documentada.

---

# 67. REGRA DE TESTES DE VEÍCULO

Durante teste em veículo:

- prioridade à segurança;
- não interagir com sistemas críticos;
- não exigir operação visual insegura durante movimento;
- executar cenários controlados;
- parar teste se hardware apresentar comportamento anormal.

---

# 68. REGRA DE EXTERNAL PLATFORM

Google, Apple, OEMs e vendors são dependências externas.

O núcleo TPS não deve perder independência por conveniência de integração.

---

# 69. REGRA DE PRODUÇÃO

Depois de `PRODUCTION_READY = TRUE`:

1. congelar v1.0.0;
2. criar tag;
3. armazenar evidências;
4. gerar as-built;
5. abrir backlog pós-produção;
6. somente então planejar v1.1/v2.

---

# 70. MANIFESTO FINAL

Este projeto deve ser concluído por progressão linear controlada.

```text
ONE PROJECT
ONE ROADMAP
ONE ACTIVE GATE
ONE CANONICAL STATE
ZERO UNAUTHORIZED BRANCHES
```

Sempre que surgir uma nova ideia:

```text
NEEDED_NOW?
  |
  +-- YES --> resolve for current gate
  |
  +-- NO --> backlog
```

Sempre que surgir uma decisão:

```text
DECIDE
FREEZE
CONTINUE
```

Sempre que faltar evidência:

```text
UNKNOWN
```

Sempre que o gate passar:

```text
PASS
FREEZE
NEXT
```

E até a produção:

```text
FINISH > EXPAND
```

---

# 71. COMANDO OPERACIONAL PARA QUALQUER NOVA SESSÃO

Copiar junto com este arquivo, ou usar como mensagem inicial:

```text
AUTORIDADE: TPS-AUTOLINK-PROJECT-CONTROL.md

Leia o documento canônico integralmente antes de agir.

Regras obrigatórias:
- Execute somente ACTIVE_GATE.
- Não crie novos gates.
- Não altere roadmap.
- Não reabra FROZEN.
- Não inicie FUTURE_GATE.
- Classifique toda descoberta como BLOCKER, NON_BLOCKING,
  FUTURE_GATE ou OUT_OF_SCOPE.
- Sem evidência: UNKNOWN/NOT_PROVEN.
- Código final deve ser validado antes de ser apresentado como produção.
- Ao atingir EXIT_CRITERIA: PASS, FREEZE, NEXT.
- Até AUTO-18: FINISH > EXPAND.

Antes de responder, informe:
PROJECT / ACTIVE_GATE / TARGET / SCOPE / BLOCKERS.

Ao terminar, informe:
PROJECT / ACTIVE_GATE / STATUS / PROVEN / PENDING /
BLOCKERS / NON_BLOCKING / FROZEN / ARTIFACTS / NEXT_EXACT_ACTION.
```

---

# 72. ASSINATURA LÓGICA DO CONTROLE

```text
DOCUMENT: TPS-AUTOLINK-PROJECT-CONTROL.md
VERSION: 1.0.0
STATUS: CANONICAL
ROADMAP: AUTO-00 -> AUTO-18
ACTIVE_GATE: AUTO-01
BRANCH_BUDGET: 0
PRODUCTION_BEFORE_ENHANCEMENT: TRUE
DEFAULT_AUTHOR_LANGUAGE: RUST
FAIL_CLOSED_ON_UNKNOWN: TRUE
PRINCIPLE: FINISH > EXPAND
```

**Fim do documento canônico v1.0.0.**
