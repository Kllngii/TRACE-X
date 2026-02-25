# TRACE-X
**T**ransit **R**oute **A**nalysis &amp; **C**apture **E**ngine for Mr. **X**

## Spielprinzip und Annahmen

### Annahmen und Grundprinzip
- Gegeben ist ein Netz aus Stationen der Verbindungstypen
  - Bahn
  - Bus
  - Fähre
- *N* Mr. X bewegen sich gleichzeitig durch das Netz
- Jeder Mr. X hat eine zugewiesene Startstation

### Bewegungsregeln Mr. X
1. Ein Mr. X darf nur eine bestimmte Anzahl an Stationen am Stück fahren
   - *X* Stationen Bahn
   - *Y* Stationen Bus
   - *Z* Stationen Fähre
2. Nach dieser Anzahl muss ein Mr. X umsteigen
3. Umsteigen ist jederzeit erlaubt, auch vor Erreichen der maximalen Anzahl an Stationen
4. Mr. X darf nicht zwischen zwei Standortmeldungen auf der selben Linie umkehren/zurückfahren
5. Alle *K* Minuten muss jeder Mr. X eine Standortmeldung abgeben
   - aktuelle Station
   - aktuelle Linie (falls in Bewegung)
   - Fahrtrichtung / Endhaltestelle der Linie (falls in Bewegung)

### Fang-Regeln
- Ziel ist es möglichst viele Mr. X zu fangen
- Derselbe Mr. X darf
  - erst nach *A* Minuten erneut gefangen werden
  - wird zwischendurch ein anderer Mr. X gefangen, reduziert sich die Schonfrist auf *B* Minuten
- Mr. X wird vor einem Fänger in Sichtweite nicht wegrennen/in ein Verkehrsmittel einsteigen und flüchten

## Usage and build instructions
![Platform Darwin](https://img.shields.io/badge/platform-macOS-orange.svg)
![Platform Windows](https://img.shields.io/badge/platform-Windows-orange.svg)

![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)
![Apache-2.0 License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)

![Version](https://img.shields.io/badge/version-0.0.1-yellow.svg)

**TODO**
