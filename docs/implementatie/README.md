# Implementatie

Bij de implementatie van het platform staat [open source](open-source.md) centraal.
Hierdoor is het platform onafhankelijk en transparant,
kan het makkelijk worden gekopieerd en aangepast,
en worden de kosten laag gehouden.

Het kennisportaal wordt gebouwd met [Material for MkDocs](//squidfunk.github.io/mkdocs-material),
waarbij de content wordt geschreven in [Markdown](../appendix/markdown.md)
en wordt gehost op [GitHub](../appendix/github.md).
Het gehele proces voor het toevoegen en wijzigen van content verloopt ook via GitHub,
en is dus volledig openbaar en transparant.
Op de pagina's kunnen opmerkingen worden geplaatst via [Giscus](https://giscus.app),
en ingelogde gebruikers kunnen daarnaast ook op het [forum](#forum) verder discussiëren.

Het forum is gebouwd met [Discourse](https://discourse.org),
de go-to opensourcesoftware voor online fora.
Omwille van de privacy en veiligheid van de gebruikers
is de inhoud van het forum niet openbaar,
maar hiervan kan naar wens wel makkelijk een kopie worden gemaakt.

## Veiligheid

Om de veiligheid van het platform te waarborgen,
worden de volgende maatregelen genomen:

- Het platform wordt gehost op een beveiligde server die regelmatig wordt geüpdatet.
  De toegang tot deze server is beperkt en beveiligd via versleutelde SSH-verbindingen.
  De server zelf wordt beveiligd met onder andere een firewall
  en bekende systemen zoals `fail2ban`, `PSAD`, `Lynis`, `logwatch` en `Wazuh`.
- Voor het broncodebeheer wordt gebruikgemaakt van de bestaande en
  wereldwijd gebruikte infrastructuur van GitHub, onderdeel van Microsoft.
- Wachtwoorden van gebruikers worden versleuteld opgeslagen
  als salted `PBKDF2` hashes met een `sha256` hashfunctie en 600,000 iteraties.
  Meerfactorauthenticatie is verplicht voor alle gebruikers.
