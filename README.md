# Programmentwurf Guardian-Backup
Karl Mörtzschky
Benedikt Kuder

07.06.2024

# Kapitel 1 - Einführung

## Übersicht über die Applikation
> Was macht die Applikation? Wie funktioniert sie? Welches Problem löst sie/welchen Zweck hat sie?
Bei Guardian-Backup handelt es sich um eine pseudeo-offline Backup-Lösung.
Dabei erstellt ein client ein Backup und lädt dieses auf den Server hoch.
Der Client kann allerdings nur neue Backups erzeugen und keine Löschen.
Das Löschen von Backups wir durch eine Retention-Policy umgesetzt.
Durch die Inhaltsaddresierung der einzelnen Dateien findet automatisch eine Deduplizierung statt.

## Wie startet man die Applikation?
> Wie startet man die Applikation? Welche Voraussetzungen werden benötigt? Schritt-für-Schritt-Anleitung
### Bauen der Artefakte
Für das Bauen der Artefakte ist eine Rust-Toolchain auf aktueller Version (>= 1.78.0) notwendig.
Diese kann mit [Rustup](https://rustup.rs/) Installiert werden.

Das Binary kann mithilfe von folgendem Befehl generiert werden:
```sh
cargo build --release --package guardian-backup-plugin-client --bin guardian-backup-plugin-client
```
Das Server Binary wiederum wird mit folgendem Befehl erzeugt:
```sh 
cargo build --release --package guardian-backup-plugin-client --bin guardian-backup-plugin-client
```

Die Artefakte befinden sich darauf in `target/release`

### Verwendung
Starte zuerst den Server indem du das Binary guardian-backup-server ausführst

Die Grundfunktionen können über folgende Befehle ausgeführt werden:

# Ganz fettes TODO

## Wie testet man die Applikation?
> Wie testet man die Applikation? Welche Voraussetzungen werden benötigt? Schritt-für-Schritt-Anleitung

Um die Anwendung zu testen kann man in dem Projektordner folgenden Befehl ausführen:
```sh
cargo test --workspace
```

# Kapitel 2 - Clean Architecture

## Was ist Clean Architecture?
> allgemeine Beschreibung der Clean Architecture in eigenen Worten

Unter Clean Architecture versteht man eine Sammlung an Konzepten, welche darauf abzielen, komplexe Softwaresysteme in leichter verständliche Komponenten aufzuteilen.

Im Wesentlichen wird die Anwendung dabei in mehrere Schichten unterteilt.
Jede Schicht greift dabei nur auf die Schichten welche tiefer liegen zu.

In der äußersten Schicht (Plugin-Schicht) werden dann Abhängigkeiten in die unterliegenden Schichten injiziert und die Benutzerschnittstelle definiert.

Darunter liegt eine Adapter-SChicht welche in diesem Projekt in die Plugin-Schicht integriert wurde.
Hier werden Adapter definiert, um externen Bibliotheken ein einheitliches Interface zu geben, sodass die Abhängigkeiten später einfach ausgetauscht werden können.

Unter der Adapter-Schicht liegt die Anwendungs-Schicht in welcher Anwendungslogik definiert wird.
Hier werden User-Stories Implementiert, welches nicht mehr Teil der Domänenlogik sind.

In der für dieses Projekt tiefsten Schicht wird die Domänenlogik selbst implementiert.
Dabei handelt es sich um Logik welche von einem Domänenexperten vollständig beschrieben werden kann.
Es werden also Abläufe und Zusammenhänge so abgebildet wie diese auch in der Domäne selbst stattfinden.

## Analyse der Dependency Rule
> (1 Klasse, die die Dependency Rule einhält und eine Klasse, die die Dependency Rule verletzt);   jeweils UML der Klasse und Analyse der Abhängigkeiten in beide Richtungen (d.h., von wem hängt die Klasse ab und wer hängt von der Klasse ab) in Bezug auf die Dependency Rule
### Positiv-Beispiel: Dependency Rule
![UML-Diagramm](https://www.plantuml.com/plantuml/svg/bL7DIiGm4BxdAUReAka359GLyU1Dq7l9PiSQDftKoRHbKT_TQQneOVVWeULZlf-PhzrcSIn35utnqUKCaWuXrJMXrFMpxv-4qNNmRW0iL5K5O1HUWKxt5vi29mw22rQ5eazZUkSFhaLuZ4CRIFrB_1o9F2BV9IBq4iOWHwLdinnxX7lcn70z2_zAohYeCjA-ONmRC4lPkXurKv3NPXVJRcFAqIioa22JYyLeWT5Z657ft2VAfSLYAH7hBhSqAJ5G-rldvyH4twqUvlmNvt5LwT-tBulrqVVfJsRjnytvzMTCbzQU_i5cEpVhuXS0)  

Die Klasse `TcpServerConnectivity` (Plugin-Schicht) implementiert `ConnectionServerInterface` (Anwendungs-Schicht) und kann so in der Anwendungs-Schicht verwendet werden, obwohl die TCP-Implementation tokio als externe Bibliothek verwendet.
### Negativ-Beispiel: Dependency Rule
![UML-Diagramm](https://www.plantuml.com/plantuml/svg/PP5BQiGm343tFeMPLKkP2uGXJ4isini8T1kChIIYZXpywPH-RjERz69r1iNzH0b4Ua_4mZg6g7nCanK2AOBlbGOPm1kegXOyAekli5KDX2A5c9L-KWF8iqq3adpx8VTq0JA9Xj-mSTFN9y62j5KXo8SiqfnhcMClqLICJeQHWMVKKCdc-ZqPe8WzWcSTz0WcsTpGSiQYHmkTToxcFxgNmrd5iMOLICrXT35KyX7qeUMo5bT_9makDxUSAsEwjRz0D6lAuwrb6QU0CaOXvVH2aABhjdW2Fy0V_Y7okpo2xss_Vw_bxl_ITblDYoiDEOxBUYRx3G00)

Die Klasse `Backup` (Domänen-Schichte) implementiert `Serialize` und `Deserialize`. Beide Interfaces werden in der externen Bibliothek serde definiert. Dieser Bruch der Dependency rule wird hingenommen, da serde als sehr stabil gilt und es als unwahrscheinlich anzunehmen ist, dass die verwendeten Interfaces sich noch ändern. Eine Re-Implementation von serde-Interfaces schien nicht sinnvoll.


## Analyse der Schichten
> jeweils 1 Klasse zu 2 unterschiedlichen Schichten der Clean-Architecture: jeweils UML der Klasse (ggf. auch zusammenspielenden Klassen), Beschreibung der Aufgabe, Einordnung mit Begründung in die Clean-Architecture
### Schicht: [Domäne]
![UML-Diagramm](https://www.plantuml.com/plantuml/svg/NL1DQiKW4DvxYbacBRb0A2MBdY0jtOheb0XgWevfeVJkdOY-l9U5yFteDtup9LAHidT2QATIWdUzeCaEuLS0310PfX4-KRyqP-RpAjXzXe3VNGy5bejTNx0oHXwyqeX-tR4g8Fwke-PpN0fgIyjAqjal9EjnXBSS5Tar5Dy6mhWhTv4vZYJ-eCw7DC87l-HYVhlxPa4jjj8Mrzmp1atL4fAywjpWjgQdONWe8iI4mV12_m40)  

Die Klasse `Schedule` beschreibt den Rhythmus in welchem Backups erstellt werden sollen.  
Dieser Plan besteht aus mehreren `ScheduleRule` welche jeweils eine Regel sowie deren letzte Ausführung beschreiben.  
Hier handelt es sich im Domänenlogik, da die Aufgabe darin liegt einen Backup-Plan (ein Objekt aus der Domäne) zu repräsentieren.  

### Schicht: [Plugin]
![UML-Diagramm](https://www.plantuml.com/plantuml/svg/ZP5BJiCm48RtFiKiUJGNgAgeL2p8eXiarf4RJuD5_90zLgc0st0JBeP9J92h968t-VBDv9_9rXDt55J76BEyUU4jmfq-eHP1j_36cevx3vNUepBkl8j6i8zeZPSKdwC0KC2XntuD3zm_xxdTF7bqbK1DyYZHhKO-z5TY1KDLbem1t4ABLASXLzSNf3wSQoaI3bPmmlGciSPXndgeyVHJie-_jUwD-lhKs6UMcj0TUOCmkT9KfjbKqxccaW74E033JNzlCNvGYZNipEl6aojpDJ_dGaC-Eub6AKaeUYCwi_-oADvBkAHB-isumxALn__FHBP5BOBglW40)

Die `InMemoryBackupRepository`-Klasse ist eine Implementierung des `BackupRepository`-Interfaces.  
Sie speichert Backup-Objekte direkt im Arbeitsspeicher.  
Da es sich bei der Art der Speicherung um ein Implementierungsdetail handelt ist diese Klasse in die Anwendungs-SChicht einzuordnen.  



# Kapitel 3 - SOLID
## Analyse Single-Responsibility-Principle (SRP)
> jeweils eine Klasse als positives und negatives Beispiel für SRP;  jeweils UML der Klasse und Beschreibung der Aufgabe bzw. der Aufgaben und möglicher Lösungsweg des Negativ-Beispiels (inkl. UML)
### Positiv-Beispiel
![UML-Diagramm](https://www.plantuml.com/plantuml/svg/dOzFYy8m5CJl-HJlkClIsx8iHKhjGV3WHRo9b3HzrM2I9_a3LkrtjxHOQZtfBR_9C3ClTPvR7xHYqdYq5HS8cQ9YWLuSuCe0Vi2Yvj98iyyLcg_lJlFakHgnHLwEdhcd7AbgbOOn9XHt3fPcXPfi_HmVGT5o31cTPzlmlqlOkvtDes13HqcJzCw4DWQfJmhchZPYtIhTLv_dv0LZ27-_Wxsd3sPC1aJHBO41sUJ-LqEAfWnQtG40)

Das Interface `BlobFetch`, bzw. die Implementierungen `InMemoryBlobFetch` und `TokioBlobFetch` sind dafür zuständig BLOBs zu repräsentieren.
Dabei muss im Fall des `TokioBlobFetch` der BLOB selbst zu keinem Zeitpunk vollständig im Arbeitsspeicher vorliegen.
Das SRP ist erfüllt, da es nur die Funktion des Lesens eines BLOBs implementiert.

### Negativ-Beispiel
![UML-Diagramm](https://www.plantuml.com/plantuml/svg/ROmn2a8n341tJv5H4I_WwEwY1oZvDMWmFwrfloBYtKqjYCDVGdXvBr6m5DWZwv7iJjOcHuBN0c030yRhb8DHJeLhikSMUCm2koy__72N9GqpgjC_qSqrN51FGe4rff7rxD5jebANgtvMUlZQNh9MCaK9lN3w_W00)  

Die Klasse `MainServerService` ist dafür zuständig eingehende Anfragen zu bearbeiten. Dabei ist vor allem die Methode `internal_handle()` problematisch, da sie die Logik für alle Arten an Anfragen implementiert.
So ist das Prinzip der Single Responsibility gebrochen.
Ein Lösungsansatz dafür, wäre es die Logik zur Abarbeitung der Anfragen auszulagern. Beispielsweise könnte diese Logik direkt als Methode `handle()` in der Anfrage selbst implementiert werden.  
![Lösungsansatz UML](https://www.plantuml.com/plantuml/svg/TOwnJiKm34NtV8L74EeFCA0d69XOGCoLcreGuJWXSK3emR-JO1KL0KjLtNFkZGzLiMYBx3nZhN23GUwel50Pt-09ZWvWWWNzKyjpePngq5JUpY74p73vbTz-noPpMlvGUxeJkta6ZoWhqnp4fnZaePUU6rKzEJKizDf_nsGGfIB8ipLw-k4SBbKBUWMx_Ih5s2aSTzD3CtfDEYxuGMt6-7tcXEi_obQUflUBxHrdZT_vWMHjaJy1)

## Analyse Open-Closed-Principle (OCP)
> jeweils eine Klasse als positives und negatives Beispiel für OCP;  jeweils UML der Klasse und Analyse mit Begründung, warum das OCP erfüllt/nicht erfüllt wurde – falls erfüllt: warum hier sinnvoll/welches Problem gab es? Falls nicht erfüllt: wie könnte man es lösen (inkl. UML)?
### Positiv-Beispiel
![UML-Diagramm](https://www.plantuml.com/plantuml/svg/JSynhW8X4CRndbDCzVBKS05ZhBStiTsCinzPafKb0sfXlBk8KzZ1V-17a8LBeO7BV8BUeu2UZiXJkUMyQiKqpcq6BNiwGmxqD_Gj7LqxjgE4n_-chx8Y1AXTi1XYcaPPhvbhl2mmgmrVpiLKeO45PEJdOOEWI8AEj2ASleZ2SEiAhfOw-VkDhVS6lm40)  
Die Klasse `HashService` verwaltet verschiedene `Hasher` und hat die Aufgabe je nach Situation den korrekten `Hasher` auszuwählen.
Das OCP ist erfüllt da der `HashService` geschlossen gegen Änderungen ist (das Hinzufügen einer neuen `Hashers` ist alleine über das Hinzufügen eines Objektes in `supported_hashers` möglich).
Dennoch ist der `HashService` offen, da weiterhin neue `Hasher` hinzugefügt werden können.

### Negativ-Beispiel
Die Klasse `MainServerService` zusammen mit dem Enum `Call` erfüllen nicht das OCP, da das Hinzufügen einer neuen `Call`-Variante eine Abänderung des Switch-Statements innerhalb `MainServerService.internal_handle()` notwendig ist.

aktueller Pseudo-Code:
```rust
async fn internal_handle(
        &mut self,
        call: &mut impl IncomingCall,
        call_variant: Call,
    ) -> Result<(), ServerServiceError> {
        let user = call.user();

        match call_variant {
            Call::CreateBackup(backup) => {...}
            Call::GetBackups => {...}
            Call::PatchBackup(mut backup) => {...}
            Call::CreateBlob(id) => {...}
            Call::GetBlob(id) => {...}
        }
    }
```

Dies könnte wie im SOLID Negativ-Beispiel beschrieben durch das Auslagern der Logik in den `Command` selbst gelöst werden.
Der verbesserte PseudoCode würde dann folgendermaßen aussehen:

```rust
async fn internal_handle(
        &mut self,
        call: &mut impl IncomingCall,
        call_variant: Call,
    ) -> Result<(), ServerServiceError> {

        call_variant.handle(call.user())
    }
```

## Analyse Liskov-Substitution- (LSP), Interface-Segreggation- (ISP), Dependency-Inversion-Principle (DIP)
> jeweils eine Klasse als positives und negatives Beispiel für entweder LSP oder ISP oder DIP);  jeweils UML der Klasse und Begründung, warum man hier das Prinzip erfüllt/nicht erfüllt wird
> Anm.: es darf nur ein Prinzip ausgewählt werden; es darf NICHT z.B. ein positives Beispiel für LSP und ein negatives Beispiel für ISP genommen werden
### Positiv-Beispiel
![UML-Diagramm](https://www.plantuml.com/plantuml/svg/dOzFYy8m5CJl-HJlkClIsx8iHKhjGV3WHRo9b3HzrM2I9_a3LkrtjxHOQZtfBR_9C3ClTPvR7xHYqdYq5HS8cQ9YWLuSuCe0Vi2Yvj98iyyLcg_lJlFakHgnHLwEdhcd7AbgbOOn9XHt3fPcXPfi_HmVGT5o31cTPzlmlqlOkvtDes13HqcJzCw4DWQfJmhchZPYtIhTLv_dv0LZ27-_Wxsd3sPC1aJHBO41sUJ-LqEAfWnQtG40)  
Das Interface `BlobFetch` ist ein gutes Beispiel für eine gelungene Liskov-Substitution.
Die Klassen `InMemoryBlobFetch` und `TokioBlobFetch` implementieren `BlobFetch` und erfüllen diesen Vertrag vollständig. Sie können also gegen einender ausgetauscht werden, ohne dass ein aufrufendes Modul eine Änderung des Verhaltens beobachten kann.

### Negativ-Beispiel
![UML-Diagramm](https://www.plantuml.com/plantuml/svg/dP71QiCm38RlVWeTMuPVmB1HA3lqiDlkmN5LPgRAcQLqb6tllY2wh86CbVXaVdtohr_UYCQg_P5SPiLwY0bXMvWnQIwyfOoikx7ouM0uTw3d3k6nrb83YEv3GBi7azm54kHzK_6jHz7LUaPithE-D2sLThK6z-LSeYW2pwdxdM364kwlaLFMiyd6UR_4_BhLJodR2aVwWQ1Ymdp20P2kiA0LnhRBUsEMfv9U_mkpuqCaoYtXyWXcELsSzvyFVt-biybkfZU3RN-aRm00)  

Das Interface `ConnectionClientInterface` wird von `TcpConnection` und `NockConnection` implementiert.
Aus der Natur einer Mock-Implementation heraus, bricht `MockConnection` den in `ConnectionClientInterface` vereinbarten Vertrag, indem z.B. der Rückgabewert der Funktionen unabhängig von den Parametern dieser sind.
So kann auf ein `Command::GetBlob` mit einer `Response::BackupList` zurückgegeben werden. Dies ist in einer Anwendung in welcher alle Komponenten sich verhalten wie spezifiziert nicht möglich. Eine `TcpConnection` kann also nicht einfach durch eine `MockConnection` ausgetauscht werden.

Da dieses Verhalten im Rahmen von Unit-Tests gewünscht ist, wird hier kein wirklicher Lösungsvorschlag gegeben.

## Kapitel 4 - Weitere Prinzipien
### Analyse GRASP: Geringe Kopplung
> jeweils eine bis jetzt noch nicht behandelte Klasse als positives und negatives Beispiel geringer Kopplung; jeweils UML Diagramm mit zusammenspielenden Klassen, Aufgabenbeschreibung und Begründung für die Umsetzung der geringen Kopplung bzw. Beschreibung, wie die Kopplung aufgelöst werden kann
### Positiv-Beispiel
### Negativ-Beispiel

## Analyse GRASP: Hohe Kohäsion
> eine Klasse als positives Beispiel hoher Kohäsion; UML Diagramm und Begründung, warum die Kohäsion hoch ist
## Don’t Repeat Yourself (DRY)
>ein Commit angeben, bei dem duplizierter Code/duplizierte Logik aufgelöst wurde; Code-Beispiele (vorher/nachher); begründen und Auswirkung beschreiben


# Kapitel 5 - Unit Tests
## 10 Unit Tests
> Nennung von 10 Unit-Tests und Beschreibung, was getestet wird+

Unit Test | Beschreibung
-|-
server::TcpConnection::test_receive_request | testet ob der Server eingehende `Command`s empfangen und richtig parsen kann
server::TcpConnection::test_send_response | testet ob der Server ausgehende `Response`ses korrekt encoded und sendet
server::TcpConnection::test_receive_blob | testet ob der Server bei eingehenden `Commands`s auch BLOBS korrekt empfangen kann
server::TcpConnection::test_send_response_with_blob | testet ob der Server bei ausgehenden `Response`s BLOB korrekt senden kann 
client::TcpConnection::test_send_request() | testet ob der Client `Command`s korrekt encoden und senden kann und ob Responses korrekt empfangen werden
client::TcpConnection::test_send_request_blob() | testet ob der Client bei `Command`s BLOBs korrekt mitsenden kann und ob Responses korrekt empfangen werden
client::TcpConnection::test_receive_blob() | testet ob der Client Responses mit BLOBs korrekt empfangen kann

## ATRIP: Automatic
> Begründung/Erläuterung, wie ‘Automatic’ realisiert wurde

Die Tests wurden mithilfe von Cargo tests umgesetzt.
So können durch `sh cargo tests` automatisiert alle tests gestartet werden und Ergebnisse werden in der Konsole zusammengefasst.
Eine integration in gängige IDEs ist hierdurch auch gegeben.
Durch den Rückgabwert des Prozesses können die tests einfach in Pipelines eingebaut werden.

## ATRIP: Thorough
> jeweils 1 positives und negatives Beispiel zu ‘Thorough’; jeweils Code-Beispiel, Analyse und Begründung, was professionell/nicht professionell ist

### Positives Beispiel
In `server::TcpConnection` wurde ‘Thorough’ getestet.
Eine Verbindung kann nur `Command`s empfangen und `Request`s senden.
Beide aktionen sind mit und ohne BLOB möglich. Es gibt also insgesamt nur vier mögliche Aktionen.
Die Unit-Tests decken alle vier Fälle ab und sind damit ‘Thorough’.

### Negatives Beispiel

# fettes TODO

## ATRIP: Professional
> jeweils 1 positives und negatives Beispiel zu ‘Professional’; jeweils Code-Beispiel, Analyse und Begründung, was professionell/nicht professionell ist

### Positives Beispiel
Als positives Beispiel dient `server::TcpConnection::test_receive_request()`.
Die Presstonalität ist hier aus mehreren Gründen gegeben:
1. Der Name beschreibt was die Funktion testet
2. Bestehender Code wird wiederverwendet (send_call)
3. Der Test testet tastsächliche Logik und Interaktion zwischen Modulen und ist damit sinnvoll.

### Negatives Beispiel

# fettes TODO

## Code Coverage
> Code Coverage im Projekt analysieren und begründen

# fettes TODO 

## Fakes und Mocks
> Analyse und Begründung des Einsatzes von 2 Fake/Mock-Objekten; zusätzlich jeweils UML Diagramm der Klasse

### Beispiel 1
![UML_Diagramm](https://www.plantuml.com/plantuml/svg/SoWkIImgAStDuShCAqajIajCJbNmI2pEI2rIgEPI009jXPBAWbI5WDIybCoyT92KDHTKeg0eDIsrA3KlELL34ogKd9WNdvoVMv1Ob1gV0LIBa2XAJIo1YzLoSINd91ONA_Zc9sVJnJeaYtHrQ-oWVkHo0De3z3m0)

Die Klasse `MockHasher` stellt eine Mock-Implementation des `Hasher`-Interfaces dar.
Sie wurde erstellt damit Komponenten getestet werden können obwohl eine vollständige Implementation des Hasher-Interfaces noch nicht existierte.

# TODO Wo Einsatz?

### Beispiel 2


# Kapitel 6 - Domain Driven Design
## Ubiquitous Language
> 4 Beispiele für die Ubiquitous Language; jeweils Bezeichung, Bedeutung und kurze Begründung, warum es zur Ubiquitous Language gehört
Bezeichung | Bedeutung | Begründung
-|-|-

## Entities
> UML, Beschreibung und Begründung des Einsatzes einer Entity; falls keine Entity vorhanden: ausführliche Begründung, warum es keines geben kann/hier nicht sinnvoll ist
## Value Objects
> UML, Beschreibung und Begründung des Einsatzes eines Value Objects; falls kein Value Object vorhanden: ausführliche Begründung, warum es keines geben kann/hier nicht sinnvoll ist
## Repositories
> UML, Beschreibung und Begründung des Einsatzes eines Repositories; falls kein Repository vorhanden: ausführliche Begründung, warum es keines geben kann/hier nicht sinnvoll ist
## Aggregates
> UML, Beschreibung und Begründung des Einsatzes eines Aggregates; falls kein Aggregate vorhanden: ausführliche Begründung, warum es keines geben kann/hier nicht sinnvoll ist

# Kapitel 7 - Refactoring
## Code Smells
> jeweils 1 Code-Beispiel zu 2 Code Smells aus der Vorlesung; jeweils Code-Beispiel und einen möglichen Lösungsweg bzw. den genommen Lösungsweg beschreiben (inkl. (Pseudo-)Code)]
## 2 Refactorings
> 2 unterschiedliche Refactorings aus der Vorlesung anwenden, begründen, sowie UML vorher/nachher liefern; jeweils auf die Commits verweisen]

## Kapitel 8 - Entwurfsmuster
> 2 unterschiedliche Entwurfsmuster aus der Vorlesung (oder nach Absprache auch andere) jeweils sinnvoll einsetzen, begründen und UML-Diagramm]
### Entwurfsmuster: [Name]
### Entwurfsmuster: [Name]
 
