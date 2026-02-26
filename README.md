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
Ein Netzwerk hart zu kodieren ist nicht sinnvoll, deshalb enthält das `network`-module `Parser`, um Netzwerke aus einer externen Datei einzulesen.

#### JSON-Parser
Die JSON (**J**ava**S**cript **O**bject **N**otation) ist eine kompakte und leicht lesbare Notation für den Datenaustausch zwischen Anwendungen und eignet sich somit sehr gut zur Angabe kleinerer Netzwerke zu Testzwecken.

Der Einfachkeit halber wurde festgelegt, dass jede per JSON-Parser eingelesene Verbindung stets eine Gegenrichtung besitzt.
Gibt es auf der Linie 1 eine Verbindung von Station A zur benachbarten Station B, ist es ebenfalls möglich die Gegenrichtung von Station B nach Station A ohne Zwischenstopp zu fahren.
Für Menschen mag das selbstverständlich wirken, der Algorithmus wird aber mit gerichteten Kanten (directed edges) arbeiten und muss explizit beide Fahrtrichtungen angegeben bekommen.

Ebenfalls festgelegt wurde, dass die Daten des Netzwerks linienbasiert und nicht stationsbasiert eingelesen werden, obwohl der stationsbasierte Ansatz eine größere Ähnlichkeit von JSON und Netzwerk zur Folge hätte.
Bei größeren Netzwerken mit mehreren Linien spart der linienbasierte Ansatz ganz einfach Schreibarbeit und ist ebenfalls besser für Menschen mit einem Fahrplan auf Vollständigkeit und Richtigkeit zu prüfen.

##### Aufbau eines Line-Objekts
Das Parsen wird von dem Rust-crate `serde` und der dazugehörigen `serde_json` Erweiterung übernommen.

```rust
#[derive(Deserialize)]
struct JsonLineInput {
    name: String,
    transport: String,
    stations: Vec<String>,
}
```
Beispielhaft für vier benachbarte Stationen der Hamburger Bahnlinie U1 könnte das so aussehen:
```json
{
  "name": "U1",
  "transport": "bahn",
  "stations": [
    "Wandsbek-Gartenstadt",
    "Alter Teichweg",
    "Straßburger Straße",
    "Wandsbek Markt"
  ]
}
```

##### Aufbau des JSON-Objekts
Rust-seitig ist die Gesamtstruktur nichts weiter als eine Liste der zuvor definierten Linien.
```rust
#[derive(Deserialize)]
struct JsonNetworkInput {
    lines: Vec<JsonLineInput>,
}
```
Eine vollständige JSON-Datei könnte so aussehen, der `lines`-Array enthält hierbei nach dem oben genannten Schema definierte Linien.
```json
{
  "lines":[]
}
```

#### GTFS-Parser
GTFS (**G**eneral **T**ransit **F**eed **S**pecification) ist ein Standardformat für den Austausch von Fahrplandaten inlusive Abfahrtszeiten, Namen und Standorten der einzelnen Haltestellen.
Ein Beispieldatensatz für Hamburg ist im Transparenzportal der Stadt Hamburg (https://suche.transparenz.hamburg.de/dataset/hvv-fahrplandaten-gtfs-januar-2025-bis-dezember-2025) zu finden.

Stand jetzt ist der GTFS-Parser noch nicht implementiert.

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

### Eingabe einer Standortmeldung
Standortmeldungen werden alle *K* Minuten abgegeben und liegen in folgendem Format vor:
```regex
\`d{2}:\d{2}/Mr.X\d/[\w\s-]+/[\w]+/[\w\s-]+
```
Eine Standortmeldung von Mr. X Nummer 1 um 10:00 könnte also so aussehen.
```text
10:00/Mr.X1/Wandsbek-Gartenstadt/U1/Norderstedt Mitte
```
Hierbei ist die erste angegebene Station der aktuelle Standort gefolgt von verwendeter Linie und Fahrtrichtung/Endhaltestelle der Linie.
Unter Umständen kann sich ein Mr. X zum Zeitpunkt der Standortmeldung auch in keinem Verkehrsmittel befinden, dann bleiben die letzten beiden Felder frei:
```text
11:00/Mr.X2/Dammtor/ /
11:00/Mr.X3/Stadthausbrücke/-/-
```
### Algorithmus
**TODO**

## Usage and build instructions
![Platform Darwin](https://img.shields.io/badge/platform-macOS-orange.svg)
![Platform Windows](https://img.shields.io/badge/platform-Windows-orange.svg)

![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)
![Apache-2.0 License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)

![Version](https://img.shields.io/badge/version-0.1.0-yellow.svg)
[![dependency status](https://deps.rs/repo/github/kllngii/TRACE-X/status.svg?path=%2F)](https://deps.rs/repo/github/kllngii/TRACE-X?path=%2F)

**TODO**
