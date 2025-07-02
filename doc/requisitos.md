# Requisitos Funcionais

## Funcionalidades Principais do Sistema

### Gestão de Utilizadores

- Registo e identificação de impressões digitais
- Gestão de perfis de utilizadores (funcionários permanentes, temporários, visitantes) ??
- Atribuição de níveis de acesso por função/departamento (Tipo porta do servidor, usar o dedo para entrar)
- Processo de remoção/desativação de utilizadores

> Nota: A gestão de users só é um requisito se por ventura não for feita por um sistema principal, como um ERP ou IDP.

### Autenticação Biométrica

- Captura e verificação de impressões digitais em tempo real
- Tempo de resposta inferior a 3 segundos
- Suporte para múltiplas tentativas
- Modo de autenticação de backup (cartão/PIN) em caso de falha biométrica
- Permissão de utilização de vários "dedos" por utilizador

### Interface de Utilizador

- Display OLED para feedback visual e instruções
- Indicadores luminosos para estados do sistema (pronto, a processar, erro)
- Mensagens de confirmação e erro em múltiplos idiomas
- Interface web para administração do sistema

### Gestão de Dados e Relatórios

- Registo timestamp de todas as entradas e saídas
- Armazenamento seguro de modelos biométricos
- Geração de relatórios de acesso por período/utilizador
- Exportação de dados para o sistema de gestão de recursos humanos

### Conectividade e Integração

- Conectividade Wi-Fi para sincronização de dados
- API REST para integração com sistemas internos
- Suporte para protocolos de comunicação seguros (HTTPS, SSL/TLS)

### Casos de Uso Específicos

#### Funcionário Regular e Temporário

- Chegada matinal: Autenticação rápida e registo de entrada
- Saída para almoço: Registo de saída temporária
- Regresso do almoço: Re-entrada automática
- Saída final: Registo de fim de expediente

#### Visitante

- Pré-registo pelo anfitrião
- Check-in com verificação de identidade??
- Emissão de cartão/badge temporário com QR code
- Notificação automática ao anfitrião interno

#### Administrador

- Gestão de eventos de segurança e alertas
- Manutenção de base de dados de utilizadores
- Configuração de parâmetros de segurança do sistema
- Adição de visitantes/temporários

## 3. Requisitos Não-Funcionais

### Desempenho

- **Tempo de Resposta**: Autenticação biométrica ≤ 2 segundos
- **Throughput**: Capacidade para processar XXX utilizadores/hora
- **Disponibilidade**: 99.5% uptime durante horário laboral
- **Capacidade**: Suporte para até XXXX Modelos biométricos armazenados

### Segurança

- **False Acceptance Rate**: ≤ 0.001%
- **False Rejection Rate**: ≤ 1%
- **Encriptação**: Modelos biométricos encriptados com AES-256
- **Auditoria**: Log completo de todas as tentativas de acesso
- **Backup**: Cópia de segurança automática diária dos dados

### Usabilidade

- **Facilidade de Uso**: Interface intuitiva sem necessidade de formação
- **Acessibilidade**: Compatível com utilizadores com deficiências físicas
- **Feedback**: Resposta visual e sonora clara para todas as ações
- **Recuperação de Erros**: Instruções claras para resolução de problemas

<!-- ### Confiabilidade e Manutenibilidade

- **MTBF (Mean Time Between Failures)**: > 8760 horas (1 ano)
- **MTTR (Mean Time To Repair)**: < 4 horas
- **Atualizações**: Capacidade de atualização remota de firmware
- **Diagnóstico**: Sistema de auto-diagnóstico com alertas proativos -->
<!--
### Especificações Técnicas do Hardware

### Microcontrolador ESP32

- Processador dual-core 32-bit
- Wi-Fi 802.11 b/g/n integrado
- Bluetooth 4.2 BR/EDR e BLE
- Memória Flash: 4MB mínimo
- RAM: 520KB
- Tensão de alimentação: 3.3V

### Sensor de Impressões Digitais

- Resolução: 500 DPI mínimo
- Área de captura: 15x20mm mínimo
- Interface: UART/Serial
- Tempo de captura: < 1 segundo
- Capacidade de armazenamento: 1000+ Modelos

### Display e Interface

- Display OLED 0.96" com resolução 128x64
- Interface I2C para comunicação
- LED de status multicolor
- Buzzer para feedback sonoro

### Alimentação e Conectividade

- Fonte de alimentação 5V DC regulada
- Backup com bateria Li-ion 3.7V
- Autonomia mínima de 8 horas em caso de falha elétrica
- Proteção contra surtos e variações de tensão -->

## 4. Necessidades mínimas de segurança

- Modelos biométricos não são recuperáveis em formato original
- Logs de auditoria são tamper-proof e com timestamp certificado
- Sistema resiste a tentativas de bypass físico documentadas
- Conformidade com regulamentações de proteção de dados (RGPD)
