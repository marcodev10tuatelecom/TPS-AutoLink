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
- recuperação;
- métricas.

### Critério de saída

Head unit acessa endpoint de teste através da conectividade fornecida pelo telefone e registra métricas.

---

## AUTO-05 — Canal de aplicação bidirecional

### Objetivo

Telefone e head unit trocam comandos e eventos de aplicação.

### Deve entregar

- framing;
- request/response;
- eventos;
- erros;
- timeouts;
- correlação;
- testes de mensagens inválidas.

### Critério de saída

Comandos e eventos trafegam de ponta a ponta, com erros e timeouts tratados.

---

## AUTO-06 — Áudio local

### Objetivo

Reproduzir áudio controlado na head unit sem depender ainda do serviço de rádio público.

### Deve entregar

- pipeline de áudio;
- play/pause/stop;
- volume conforme API permitida;
- seleção de fonte;
- tratamento de erro;
- sincronização básica.

### Critério de saída

Arquivo local aprovado toca na head unit de referência com comandos funcionais.

---

## AUTO-07 — Rádio/streaming via Internet

### Objetivo

Consumir stream de rádio/áudio aprovado através da conectividade do telefone.

### Deve entregar

- cliente de stream;
- buffering;
- reconexão;
- métricas;
- timeout;
- erro de origem;
- codec mínimo aprovado.

### Critério de saída

Stream aprovado toca, recupera falha transitória e registra métricas.

---

## AUTO-08 — Comandos de mídia

### Objetivo

Controlar mídia por UI/eventos do TPS AutoLink.

### Deve entregar

- play;
- pause;
- stop;
- next/previous quando aplicável;
- volume quando aplicável;
- estado atual;
- feedback.

### Critério de saída

Comandos aprovados alteram o estado da mídia e o estado é refletido corretamente.

---

## AUTO-09 — Recuperação de conexão

### Objetivo

Recuperar automaticamente perda transitória de conectividade.

### Deve entregar

- detecção de perda;
- backoff;
- retry;
- limites;
- recuperação de sessão;
- métricas;
- estado de erro permanente.

### Critério de saída

Falha transitória aprovada é recuperada sem intervenção manual e falha permanente não gera loop infinito.

---

## AUTO-10 — Cache local controlado

### Objetivo

Implementar cache local limitado e governado.

### Deve entregar

- política de cache;
- quota;
- TTL;
- invalidação;
- integridade;
- limpeza;
- métrica;
- comportamento offline limitado.

### Critério de saída

Cache aprovado respeita quota, TTL, invalidação e integridade.

---

## AUTO-11 — Handover de conectividade

### Objetivo

Alterar entre caminhos de conectividade aprovados preservando comportamento da aplicação quando possível.

### Deve entregar

- detecção de caminho;
- prioridade;
- mudança controlada;
- reconexão;
- métrica;
- teste Wi-Fi ↔ celular conforme suporte real.

### Critério de saída

Mudança de caminho aprovada ocorre sem corrupção de estado e com recuperação dentro do limite definido.

---

## AUTO-12 — UI touchscreen

### Objetivo

Entregar UI funcional de referência na head unit.

### Deve entregar

- tela principal;
- status;
- mídia;
- conectividade;
- erros;
- navegação mínima;
- acessibilidade básica;
- input touchscreen.

### Critério de saída

Fluxos aprovados são executáveis pela UI da head unit de referência.

---

## AUTO-13 — Android Auto / Android Automotive

### Objetivo

Integrar apenas APIs e fluxos oficialmente permitidos.

### Deve entregar

- análise oficial atual;
- requisitos de distribuição;
- permissões;
- API surface permitida;
- adaptador separado do core;
- testes suportados.

### Critério de saída

Integração autorizada funciona dentro das regras oficiais aplicáveis.

### Regra especial

Se integração exigir aprovação externa não obtida:

```text
EXTERNAL_DEPENDENCY
```

Não falsificar suporte.

---

## AUTO-14 — iPhone / TPS AutoLink

### Objetivo

iPhone se comunica com a head unit pelo TPS AutoLink próprio no escopo aprovado.

### Deve entregar

- app/adaptador Apple;
- discovery permitido;
- sessão;
- conectividade;
- comandos suportados;
- testes.

### Critério de saída

Comunicação própria aprovada funciona no hardware Apple de referência.

---

## AUTO-15 — CarPlay / MFi

### Objetivo

Investigar e integrar somente por canais oficiais e licenciados.

### Deve entregar

- requisitos oficiais;
- processo de adesão aplicável;
- arquitetura de adaptador;
- isolamento do core;
- estado da dependência externa.

### Critério de saída

Somente `PASS` se autorização, hardware e requisitos oficiais permitirem execução real.

Caso contrário:

```text
EXTERNAL_DEPENDENCY_NOT_SATISFIED
```

Sem emulação não autorizada.

---

## AUTO-16 — Segurança, robustez e fuzzing

### Objetivo

Tornar o sistema resistente a entradas inválidas e condições hostis.

### Deve entregar

- fuzzing de parsers;
- testes de malformed input;
- limites;
- rate limiting onde aplicável;
- resource exhaustion tests;
- testes de reconnect;
- verificação de logs sensíveis;
- revisão de unsafe/FFI;
- dependency audit;
- threat-model atualizado.

### Critério de saída

Zero vulnerabilidade crítica conhecida aberta no escopo testado e critérios de robustez aprovados.

---

## AUTO-17 — Carga, soak e resiliência

### Objetivo

Comprovar operação sustentada.

### Deve entregar

- soak test;
- carga;
- memória;
- CPU;
- rede;
- reconexões;
- falhas induzidas;
- recuperação;
- métricas.

### Critério de saída

Sistema opera pelo período e carga aprovados sem falha crítica, leak não controlado ou degradação fora do limite.

---

## AUTO-18 — Veículo e release candidate

### Objetivo

Executar validação física controlada e certificar a release v1.0.0.

### Deve entregar

- teste de bancada final;
- teste em veículo controlado;
- power cycle;
- suspensão/retorno quando aplicável;
- perda de telefone;
- retorno do telefone;
- perda de WAN;
- recuperação;
- áudio;
- UI;
- logs;
- release artifact;
- checksums;
- SBOM;
- as-built;
- known issues;
- rollback;
- critérios de produção.

### Critério de saída

```text
ALL_MANDATORY_TESTS = PASS
CRITICAL_BLOCKERS = 0
HIGH_BLOCKERS = 0
PRODUCTION_READY = TRUE
RELEASE = v1.0.0
```

---

# 10. MÁQUINA DE ESTADOS DOS GATES

Estados permitidos:

```text
NOT_STARTED
READY
IN_PROGRESS
BLOCKED
FAILED
PASS
FROZEN
```

Transições permitidas:

```text
NOT_STARTED -> READY
READY -> IN_PROGRESS
IN_PROGRESS -> BLOCKED
IN_PROGRESS -> FAILED
IN_PROGRESS -> PASS
BLOCKED -> IN_PROGRESS
FAILED -> IN_PROGRESS
PASS -> FROZEN
FROZEN -> <NEXT_GATE_READY>
```

Proibido:

```text
NOT_STARTED -> PASS
READY -> PASS
BLOCKED -> PASS
FAILED -> PASS
FROZEN -> IN_PROGRESS
```

salvo Change Control formal.

---

# 11. POLÍTICA DE UM ÚNICO GATE ATIVO

Em qualquer instante:

```text
ACTIVE_GATE_COUNT = 1
```

Nunca:

```text
AUTO-03 = IN_PROGRESS
AUTO-04 = IN_PROGRESS
```

simultaneamente.

Pode existir preparação documental de gate futuro somente quando:

- explicitamente autorizada;
- não consome implementação;
- não interrompe o gate atual.

---

# 12. BRANCH BUDGET

```text
BRANCH_BUDGET = 0
```

Significa:

- não criar roadmap paralelo;
- não criar subtarefas ilimitadas;
- não criar “fase alternativa”;
- não iniciar rewrite paralelo;
- não inventar arquitetura B enquanto A ainda não falhou.

Branches Git normais para desenvolvimento **não** são o mesmo conceito, mas ainda devem ser usadas de forma controlada.

---

# 13. CLASSIFICAÇÃO OBRIGATÓRIA DE DESCOBERTAS

Toda descoberta nova deve receber **uma e apenas uma** classificação:

```text
BLOCKER
NON_BLOCKING
FUTURE_GATE
OUT_OF_SCOPE
```

## 13.1 BLOCKER

Somente quando impede critério obrigatório do gate ativo.

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

## 13.2 NON_BLOCKING

Problema real, porém não impede saída do gate.

Deve ser registrado e o gate continua.

## 13.3 FUTURE_GATE

Pertence explicitamente a gate posterior.

Não implementar agora.

## 13.4 OUT_OF_SCOPE

Não pertence à versão 1.0.0.

Não implementar.

---

# 14. POLÍTICA DE BLOQUEIO

Um blocker só é válido quando existir:

```text
MANDATORY_EXIT_CRITERION
+
PROVEN_IMPEDIMENT
```

Curiosidade não é blocker.

Melhoria não é blocker.

Preferência estética não é blocker.

“Seria bom ter” não é blocker.

---

# 15. POLÍTICA DE EVIDÊNCIA

Toda afirmação técnica relevante deve ser classificada internamente como:

```text
PROVEN
INFERRED
UNKNOWN
NOT_PROVEN
```

## 15.1 PROVEN

Existe evidência reproduzível.

Exemplos:

- comando + output;
- teste automatizado;
- log;
- captura;
- hash;
- documentação primária;
- medição física.

## 15.2 INFERRED

Inferência razoável, porém não certificável.

Nunca usar `INFERRED` como `PASS`.

## 15.3 UNKNOWN

Informação ainda não conhecida.

Estado legítimo.

## 15.4 NOT_PROVEN

Foi sugerido ou alegado, mas não existe evidência suficiente.

Nunca preencher lacuna com imaginação.

---

# 16. REGRA FAIL-CLOSED

Quando informação obrigatória estiver ausente:

```text
UNKNOWN
```

Quando teste obrigatório não tiver sido executado:

```text
NOT_EXECUTED
```

Quando evidência for insuficiente:

```text
NOT_PROVEN
```

Não usar:

```text
"provavelmente funciona"
"deve funcionar"
"parece correto"
```

como conclusão de gate.

---

# 17. POLÍTICA DE DECISÕES CONGELADAS

Uma decisão `FROZEN` só pode ser reaberta por:

```text
FAILURE_EVIDENCE
SECURITY_EVIDENCE
COMPLIANCE_REQUIREMENT
HARDWARE_INCOMPATIBILITY
USER_REQUIREMENT_CHANGE
USER_AUTHORIZATION
```

Não reabrir porque surgiu uma tecnologia mais interessante.

Formato:

```text
DECISION-ID:
DATE:
GATE:
QUESTION:
OPTIONS_CONSIDERED:
DECISION:
REASON:
EVIDENCE:
STATUS: FROZEN
REOPEN_CONDITIONS:
```

---

# 18. DECISÕES INICIAIS CONGELADAS

```text
FD-001: o produto é próprio; não é clone do CarPlay.
FD-002: o núcleo autoral novo é Rust-first.
FD-003: smartphone fornece WAN.
FD-004: transportes podem incluir USB/Wi-Fi/BLE conforme gate e suporte.
FD-005: head unit Linux de referência é o alvo inicial de bancada.
FD-006: integrações Android/Apple são adaptadores, não o core.
FD-007: CarPlay/MFi somente oficialmente.
FD-008: sem bypass de DRM/proteções/licenciamento.
FD-009: funções safety-critical ficam fora da v1.0.0.
FD-010: um gate por vez.
FD-011: produção antes de melhorias.
FD-012: branch budget operacional é zero.
FD-013: descobertas não bloqueantes não interrompem o gate.
FD-014: código não é DONE sem evidência.
FD-015: simulator PASS não equivale a physical PASS.
FD-016: unsafe/FFI devem ser confinados e auditáveis.
FD-017: versão 1.0.0 é a primeira meta operacional.
FD-018: nenhuma nova feature antes de AUTO-18 PASS sem autorização.
```

---

# 19. POLÍTICA DE DÍVIDA TÉCNICA

Dívida técnica deve ser registrada quando:

- solução correta porém provisória foi usada;
- limitação é conhecida;
- melhoria não é necessária para o gate atual;
- refatoração pode ser adiada sem comprometer segurança ou corretude obrigatória.

Formato:

```text
DEBT-ID:
GATE_DISCOVERED:
DESCRIPTION:
WHY_NOT_NOW:
RISK:
TARGET_GATE_OR_POST_PRODUCTION:
STATUS:
```

A existência de dívida técnica **não autoriza** interromper o roadmap.

---

# 20. POLÍTICA DE CÓDIGO

## 20.1 Requisitos mínimos

Todo código novo deve, quando aplicável:

- compilar sem erro;
- passar `rustfmt`;
- passar `clippy` sem warnings aprovados;
- possuir testes;
- possuir tratamento explícito de erro;
- evitar `unwrap()` em caminhos de produção sem justificativa;
- evitar panic como controle de fluxo;
- validar input;
- possuir limites de recursos;
- possuir logging adequado;
- não registrar segredo;
- ser determinístico onde necessário.

## 20.2 Entrega de scripts

Nenhum script operacional deve ser entregue como tentativa.

Antes de entrega:

- sintaxe;
- análise estática disponível;
- fluxo feliz;
- falhas previsíveis;
- idempotência quando aplicável;
- rollback quando houver mutação relevante;
- evidência.

---

# 21. POLÍTICA DE TESTE

Toda feature deve ter teste proporcional ao risco.

Categorias:

```text
UNIT
INTEGRATION
PROTOCOL
NEGATIVE
FAULT_INJECTION
SECURITY
LOAD
SOAK
PHYSICAL
REGRESSION
```

Não são todos obrigatórios em todo gate.

São obrigatórios quando diretamente relacionados ao critério de saída.

---

# 22. POLÍTICA DE TESTE NEGATIVO

Não basta provar que funciona.

Quando aplicável, provar que rejeita:

- versão inválida;
- mensagem truncada;
- payload excessivo;
- peer incorreto;
- timeout;
- conexão interrompida;
- input malformado;
- sequência inválida;
- recurso indisponível.

---

# 23. POLÍTICA DE OBSERVABILIDADE

Todo componente importante deve produzir sinais suficientes para diagnóstico.

Preferência:

```text
timestamp
component
session/request id
severity
event code
message
relevant non-secret context
```

Nunca logar:

- private keys;
- tokens;
- senhas;
- secrets completos;
- dados pessoais desnecessários.

---

# 24. POLÍTICA DE DEPENDÊNCIAS

Antes de adicionar dependência nova, verificar:

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
- versão de OS;
- alimentação;
- interfaces usadas;
- distância/ambiente quando rádio;
- passos;
- timestamps;
- logs;
- resultado;
- falhas observadas;
- evidência visual quando necessária.

---

# 30. POLÍTICA DE PRODUÇÃO

`PRODUCTION_READY = TRUE` somente quando:

```text
AUTO-00..AUTO-18 = PASS/FROZEN conforme aplicável
AND
MANDATORY_TESTS = PASS
AND
CRITICAL_BLOCKERS = 0
AND
HIGH_BLOCKERS = 0
AND
SECURITY_GATE = PASS
AND
PHYSICAL_VALIDATION = PASS
AND
ROLLBACK = PROVEN
AND
ARTIFACTS = HASHED
AND
AS_BUILT = COMPLETE
```

---

# 31. POLÍTICA DE RELEASE

Toda release deve possuir:

```text
VERSION
GIT_COMMIT
BUILD_ID
BUILD_TIMESTAMP
TARGET
RUST_VERSION
DEPENDENCY_LOCK
SBOM
ARTIFACT_SHA256
TEST_REPORT
KNOWN_ISSUES
ROLLBACK_INFO
```

---

# 32. POLÍTICA DE BACKLOG

Backlog não é fila de execução imediata.

Formato:

```text
BACKLOG-ID:
DISCOVERED_AT_GATE:
CLASSIFICATION:
DESCRIPTION:
WHY_NOT_NOW:
TARGET:
```

Backlog deve ser consultado apenas:

- quando o gate alvo chegar;
- após produção;
- quando blocker justificar reclassificação.

---

# 33. POLÍTICA DE PESQUISA

Pesquisa é permitida somente quando resolve uma decisão real do gate ativo.

Pergunta obrigatória antes de pesquisar:

```text
Qual decisão concreta esta pesquisa destrava?
```

Se nenhuma:

```text
DO_NOT_RESEARCH_NOW
```

---

# 34. POLÍTICA DE DOCUMENTAÇÃO EXTERNA

Ordem de autoridade técnica:

1. padrão/especificação oficial;
2. documentação oficial do fabricante/projeto;
3. código-fonte upstream;
4. issue/bug tracker upstream;
5. pesquisa técnica reconhecida;
6. teste próprio;
7. documentação comunitária;
8. opinião.

Para comportamento crítico, evitar depender apenas de fonte terciária.

---

# 35. POLÍTICA DE SEGURANÇA

Security by design desde o primeiro gate.

Princípios:

- least privilege;
- authenticated peers;
- explicit trust establishment;
- secure defaults;
- fail closed;
- input validation;
- rate limits;
- bounded allocations;
- minimal attack surface;
- no secret logging;
- key rotation/revocation quando aplicável;
- dependency auditing;
- reproducible evidence.

---

# 36. POLÍTICA DE PRIVACIDADE

Coletar apenas dados necessários.

Classificar:

```text
PUBLIC
INTERNAL
SENSITIVE
SECRET
PERSONAL
```

Telemetria deve evitar dados pessoais desnecessários.

---

# 37. POLÍTICA DE MUDANÇA

Toda mudança que afete:

- roadmap;
- gate;
- arquitetura congelada;
- protocolo incompatível;
- segurança;
- hardware de referência;
- requisito funcional;
- critério de produção;

deve possuir Change Control.

Formato:

```text
CHANGE-ID:
DATE:
REQUESTOR:
CURRENT_STATE:
REQUESTED_CHANGE:
REASON:
IMPACT:
RISKS:
TEST_IMPACT:
DOC_IMPACT:
DECISION:
APPROVED_BY:
```

---

# 38. POLÍTICA DE HANDOFF ENTRE SESSÕES

Antes de terminar sessão, registrar:

```text
ACTIVE_GATE
GATE_STATUS
LAST_PROVEN_STATE
OPEN_BLOCKERS
NON_BLOCKING_FINDINGS
ARTIFACTS
LAST_COMMIT
NEXT_EXACT_ACTION
```

Uma nova sessão deve continuar desse estado.

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
GATE_STATUS: IN_PROGRESS

GATE_OBJECTIVE:
  Establish controlled discovery between the approved reference phone
  and the approved reference head unit.

BLOCKERS:
  - AUTO01-B001

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
  - docs/hardware/REFERENCE-HARDWARE.md
  - docs/AUTO-01-EVIDENCE.md
  - docs/state-transitions/AUTO-01-START.md

NEXT_EXACT_ACTION:
  Supply and approve the exact reference phone and head-unit hardware matrix, then implement AUTO-01 discovery.

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

---

# 43. POLÍTICA DE STATUS

Os únicos status canônicos são:

```text
NOT_STARTED
READY
IN_PROGRESS
BLOCKED
FAILED
PASS
FROZEN
UNKNOWN
NOT_PROVEN
NOT_EXECUTED
```

Evitar variantes livres como:

```text
almost done
basically ready
probably okay
```

---

# 44. POLÍTICA DE CONCLUSÃO DO GATE

Um gate só pode virar `PASS` quando existir evidência para **todos** os critérios obrigatórios.

Se qualquer critério obrigatório estiver:

```text
UNKNOWN
NOT_PROVEN
NOT_EXECUTED
FAILED
```

então:

```text
GATE != PASS
```

---

# 45. POLÍTICA DE FREEZE

Depois de `PASS`:

1. registrar commit;
2. registrar evidência;
3. registrar artefatos;
4. atualizar documento;
5. mudar gate para `FROZEN`;
6. mover `ACTIVE_GATE` para o seguinte;
7. não retornar sem causa válida.

---

# 46. POLÍTICA DE REGRESSÃO

Todo gate posterior que alterar componente de gate anterior deve executar regressão proporcional.

Falha de regressão pode reabrir decisão somente mediante evidência.

---

# 47. POLÍTICA DE ERRO DO LLM

Se o LLM perceber que:

- inventou fato;
- misturou gates;
- implementou requisito futuro;
- afirmou teste não executado;
- contradisse decisão congelada;

deve:

1. declarar o erro;
2. corrigir o estado;
3. remover conclusão inválida;
4. retornar ao gate ativo.

Não esconder inconsistência para “manter fluidez”.

---

# 48. POLÍTICA DE TEMPO

O objetivo é reduzir **tempo de conclusão**, não maximizar atividade.

O LLM deve preferir:

```text
smallest correct step
+
clear evidence
+
freeze
+
next
```

em vez de:

```text
large speculative redesign
```

---

# 49. POLÍTICA DE ESCALONAMENTO

Escalar ao usuário somente quando for necessária decisão humana real:

- custo;
- licença;
- hardware;
- credencial;
- risco;
- mudança de requisito;
- ação física;
- aprovação externa;
- operação irreversível.

Não escalar decisões técnicas normais que o gate já resolve.

---

# 50. POLÍTICA DE CUSTO

Não introduzir serviço pago, licença, hardware ou assinatura por iniciativa própria quando alternativa aprovada já resolve.

Quando custo for inevitável:

```text
COST_DECISION_REQUIRED
```

e parar antes de contratação.

---

# 51. POLÍTICA DE LICENÇA

Toda dependência incorporada deve possuir licença compatível com o modelo de distribuição pretendido.

Estado permitido quando ainda não avaliado:

```text
LICENSE_REVIEW_REQUIRED
```

Não ignorar licença porque a biblioteca é tecnicamente conveniente.

---

# 52. POLÍTICA DE PADRÕES

Usar padrão existente quando resolver o requisito melhor do que protocolo autoral.

Criar protocolo próprio somente quando houver motivo objetivo.

Para TPS AutoLink Protocol, documentar explicitamente o que é autoral e o que utiliza padrões existentes.

---

# 53. POLÍTICA DE CRIPTOGRAFIA

Não criar primitivas criptográficas próprias.

Utilizar bibliotecas e primitivas consolidadas.

Decisões criptográficas devem considerar:

- autenticação;
- confidencialidade;
- integridade;
- replay;
- forward secrecy quando aplicável;
- rotação;
- revogação;
- armazenamento de chaves.

---

# 54. POLÍTICA DE IDENTIDADE

Identidade de dispositivo deve ser separada de:

- endereço IP;
- endereço MAC;
- nome de host;
- nome amigável.

Esses valores podem mudar e não devem ser usados automaticamente como identidade criptográfica.

---

# 55. POLÍTICA DE REDE

Todo protocolo deve possuir:

- timeout;
- limite de retry;
- backoff quando aplicável;
- limite de payload;
- limite de conexões;
- tratamento de perda;
- tratamento de duplicação quando aplicável;
- versionamento.

---

# 56. POLÍTICA DE CACHE

Antes de AUTO-10:

```text
NO_DISTRIBUTED_CACHE_PROJECT
```

É permitido apenas cache temporário mínimo necessário a gates anteriores.

P2P cache regional, cache entre usuários e distribuição cooperativa são:

```text
POST_V1_OR_SEPARATE_APPROVAL
```

salvo mudança explícita de requisito.

---

# 57. POLÍTICA DE P2P

P2P não deve aparecer como efeito colateral de outro gate.

Qualquer P2P futuro deve especificar:

- NAT traversal;
- discovery;
- relay;
- abuso;
- privacidade;
- segurança;
- accounting;
- consentimento;
- bateria;
- dados móveis;
- ISP behavior.

Isso **não pertence ao roadmap atual**, salvo autorização explícita.

---

# 58. POLÍTICA DE TURN/RELAY

TURN/relay é infraestrutura operacional e possui custo/banda.

Nenhuma implementação deve ser criada durante v1.0.0 sem gate/requisito explícito.

---

# 59. POLÍTICA DE CARPLAY

É proibido tratar CarPlay como protocolo aberto a ser clonado.

Integração somente por:

- programa oficial;
- hardware autorizado quando exigido;
- documentação/licença correspondente;
- processo de certificação aplicável.

---

# 60. POLÍTICA DE ANDROID AUTO

Avaliar requisitos oficiais correntes no AUTO-13.

Não assumir que qualquer app pode projetar qualquer UI ou serviço arbitrário.

---

# 61. POLÍTICA DE ANDROID AUTOMOTIVE

Android Automotive não é sinônimo de Android Auto.

Tratar como plataforma distinta no adaptador correspondente.

Não transformar TPS AutoLink em dependente dela.

---

# 62. POLÍTICA DE APPLE

Integração iOS própria deve respeitar APIs e permissões oficiais.

Não usar APIs privadas para evitar limitações de plataforma.

---

# 63. POLÍTICA DE HEAD UNIT

A primeira head unit é referência de engenharia, não promessa de compatibilidade universal.

Portar para hardware adicional depois que o caminho de referência estiver provado.

---

# 64. POLÍTICA DE DEPURAÇÃO

Quando um teste falhar:

1. reproduzir;
2. preservar evidência;
3. localizar camada;
4. formular hipótese;
5. testar hipótese;
6. corrigir causa;
7. executar regressão;
8. atualizar evidência.

Não alterar múltiplas camadas simultaneamente sem necessidade.

---

# 65. POLÍTICA DE ROLLBACK

Mudanças operacionais e de firmware que possam impedir funcionamento devem possuir plano de rollback proporcional ao risco.

Rollback não pode depender exclusivamente de “reinstalar tudo”.

---

# 66. POLÍTICA DE CONSTRUÇÃO

Build deve registrar:

```text
RUSTC_VERSION
CARGO_VERSION
TARGET_TRIPLE
PROFILE
FEATURES
GIT_COMMIT
LOCKFILE_HASH
ARTIFACT_HASH
```

Release final deve ser reproduzível ou ter variáveis não determinísticas documentadas.

---

# 67. POLÍTICA DE SIMULAÇÃO

Mocks e simuladores devem representar interfaces, não falsificar sucesso.

Exemplo:

```text
SIMULATOR_RESPONSE=PAIRING_OK
```

prova o fluxo de software contra o simulador.

Não prova:

```text
REAL_PHONE_PAIRING=PASS
```

---

# 68. POLÍTICA DE MÉTRICAS

Ao menos para rede e mídia, registrar quando aplicável:

- connect latency;
- reconnect latency;
- bytes;
- retries;
- errors;
- buffer state;
- underruns;
- CPU;
- memory.

Não criar plataforma completa de observabilidade antes de ser requisito.

---

# 69. POLÍTICA DE DOCUMENTAÇÃO DO PROTOCOLO

A especificação do TPS AutoLink Protocol deve conter progressivamente:

- versão;
- framing;
- message types;
- estados;
- erros;
- limites;
- security properties;
- compatibility;
- exemplos;
- testes de conformidade.

Não documentar campos inexistentes como se já fossem implementados.

---

# 70. POLÍTICA DE ENCERRAMENTO DO PROJETO v1.0.0

Quando AUTO-18 passar:

```text
PROJECT_PHASE = PRODUCTION
ACTIVE_GATE = NONE
RELEASE = v1.0.0
PRODUCTION_READY = TRUE
```

Criar snapshot final:

```text
source
binary
SBOM
docs
test evidence
hardware matrix
known issues
checksums
rollback
as-built
```

Somente depois iniciar melhoria.

---

# 71. CABEÇALHO OBRIGATÓRIO PARA O LLM

Ao iniciar resposta técnica no projeto, use mentalmente:

```text
PROJECT: TPS AutoLink
ACTIVE_GATE: <AUTO-XX>
TARGET: <critério do gate>
SCOPE: ACTIVE_GATE ONLY
BLOCKERS: <list>
```

Antes de responder:

```text
IF request_required_for_active_gate:
    execute
ELSE IF blocking:
    classify BLOCKER
ELSE IF future_gate:
    backlog
ELSE:
    do_not_expand_scope
```

Ao responder ao usuário, preferir começar com:

```text
PROJECT / ACTIVE_GATE / TARGET / SCOPE / BLOCKERS.
```

Ao terminar, informe:

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
