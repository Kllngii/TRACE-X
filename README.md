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

## Eingabe, Datenlayout und Algorithmus

### Eingabe einer Verbindung
**TODO**

### Datenlayout
Ein Network enthält eine Liste aller Knoten (hier: `stations`), der Linien (hier: `lines`) und eine Liste der Verbindungen (hier: `edges`).
Über die `adjacency` Liste ist ein Zugriff auf benachbarte bzw. in einem Schritt erreichbare Knoten einfach abbildbar.
Die `_lookup` Tabellen werden verwendet, um Name und dazugehörige Id zu verknüpfen.
```rust
#[derive(Debug)]
pub struct Network {
    pub stations: Vec<Station>,
    pub lines: Vec<Line>,
    pub edges: Vec<Edge>,
    pub adjacency: Vec<Vec<StationId>>,
    pub station_lookup: HashMap<String, StationId>,
    pub line_lookup: HashMap<String, LineId>,
}
```

Eine Verbindung (hier: `Edge`) kennt stets die Id des Startknotens und die Id des Zielknotens (`StationId` `from` und `to`).
Name der dazugehörigen Linie (hier: `line`) und Art der Verbindung (Bahn, Bus oder Fähre) sind ebenfalls bekannt.
```rust
#[derive(Debug)]
pub struct Edge {
    pub from: StationId,
    pub to: StationId,
    pub line: LineId,
    pub transport: Transport,
}
```

## Usage and build instructions
![Platform Darwin](https://img.shields.io/badge/platform-macOS-orange.svg)
![Platform Windows](https://img.shields.io/badge/platform-Windows-orange.svg)

![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)
![Apache-2.0 License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)

![Version](https://img.shields.io/badge/version-0.1.0-yellow.svg)
[![dependency status](https://deps.rs/repo/github/kllngii/TRACE-X/status.svg?path=%2F)](https://deps.rs/repo/github/kllngii/TRACE-X?path=%2F)

**TODO**
