# Dossiê local de recuperação de USD₮

O utilitário cobre transferências Ethereum finalizadas, originadas da carteira
informada, cujo destino no evento Transfer é o próprio contrato oficial USD₮.
Ele gera arquivos locais para revisão pelo titular.

```powershell
node src/tether-recovery.mjs --transaction-hash $tx --owner $owner --output $newDirectory
```

Use Node.js 22 ou superior e um diretório de saída novo. Forneça os dados
do caso em tempo de execução. O pedido usa a carteira informada como
endereço solicitado para devolução. `--rpc` aceita um RPC Ethereum HTTPS
alternativo.

Para registrar a execução opcional no QVAC, use uma instalação já configurada
do adaptador Arkhe Tucano–QVAC. Passe seu diretório e configuração locais
juntos com `--qvac-root $adapterDirectory --qvac-config $configFile`.
O relatório separa a saída do modelo do pedido e só aceita o texto do
modelo se ele corresponder à frase construída com os dados observados.

Revise `tether-request.txt`, complete os eventos e tentativas de recuperação
e envie pelo canal oficial. `ownership-statement.txt` contém a declaração
exata do solicitante. A evidência QVAC registra a execução do modelo.
A decisão sobre a recuperação cabe à Tether.
[Formulário oficial](https://cs.tether.to/TR)
