<p align="center"><em><a href="README.md">English</a> ∙ <a href="README-zh-Hans.md">简体中文</a> ∙ <a href="README-ja.md">日本語</a> ∙ <a href="README-es.md">Español</a> ∙ <a href="README-ko.md">한국어</a> ∙ <a href="README-de.md">Deutsch</a> ∙ <a href="README-fr.md">Français</a></em></p>

<br>

<div align="center">
  <img src="docs/assets/icon.svg" width="88" alt="adb-on-Symbol, das eine Telefonverbindung darstellt">
  <h1>adb-on</h1>
  <p><strong>Telefonverbindung in einem einzigen Fenster.</strong></p>
  <p>Ein kleines, einfaches Desktop-Werkzeug, um ein Android-Telefon zu verbinden, ohne ein Terminal zu öffnen.</p>
  <p>Läuft unter Windows. Ein macOS-Build wird vorbereitet.</p>
</div>

> **Windows-Version verfügbar.** Laden Sie `adb-on-windows-x64.zip` aus den Releases herunter, entpacken Sie es und öffnen Sie `adb-on.exe`. Die Verbindung mit einem echten Telefon wurde über USB und WLAN geprüft. macOS-Builds werden noch nicht veröffentlicht.

<p align="center"><img src="docs/assets/windows.png" width="400" alt="Tatsächlicher Wartebildschirm von adb-on unter Windows vor der Verbindung"></p>

**Die Aufgabe ist einfach.** Starten Sie die App und verbinden Sie das Telefon. Was am PC zu erledigen ist, übernimmt adb-on; was am Telefon bestätigt werden muss, wird jeweils angezeigt.

<br>

## Inhaltsverzeichnis

- [Warum adb-on?](#warum-adb-on)
- [Download und Unterstützungsstatus](#download-und-unterstützungsstatus)
- [Erste Verbindung über USB](#erste-verbindung-über-usb)
- [Verbindung ohne Kabel](#verbindung-ohne-kabel)
- [Beim nächsten Verbinden](#beim-nächsten-verbinden)
- [Schnelle Hilfe bei Problemen](#schnelle-hilfe-bei-problemen)
- [Was wird automatisch erledigt?](#was-wird-automatisch-erledigt)
- [Maßstab für klein und schnell](#maßstab-für-klein-und-schnell)
- [Was bleibt auf meinem PC?](#was-bleibt-auf-meinem-pc)
- [Häufig gestellte Fragen](#häufig-gestellte-fragen)
- [Haftungsausschluss](#haftungsausschluss)
- [Problemmeldung und Nutzungsbedingungen](#problemmeldung-und-nutzungsbedingungen)
- [Offizielle Referenzen](#offizielle-referenzen)

<br>

## Warum adb-on?

Sie haben eine Android-App erstellt und sind in dem Moment steckengeblieben, in dem Sie Ihr Telefon verbinden wollten?

Den Installationsort von `adb` zu suchen, Befehle zu kopieren und geänderte Ports erneut einzugeben, ist nicht der Kern der App-Entwicklung. adb-on fasst nur das, was für die Verbindung nötig ist, in einem kleinen Fenster zusammen.

- **USB wird automatisch geprüft:** Unterscheidet zwischen verbunden, Bestätigung am Telefon ausstehend und keine Antwort.
- **Drahtlos nach Anleitung:** Wählen Sie die gefundene Adresse und geben Sie den 6-stelligen Code vom Telefon ein. Wenn die automatische Erkennung fehlschlägt, können Sie die Adresse direkt eingeben.
- **Beim ersten Mal Schritt für Schritt:** Führt von den Entwickleroptionen bis zur Zulassen-Schaltfläche am Telefon.
- **Zusammen mit vorhandenen Werkzeugen:** Verwendet den Standard-ADB-Server. Ein vorhandener Server wird nicht bedingungslos beendet, und Authentifizierungsschlüssel werden nicht gelöscht.
- **Nur die nötigen Funktionen:** Konzentriert sich auf die Verbindung, ohne Cloud-Server, Bildschirmspiegelung oder Dateimanager.

<br>

## Download und Unterstützungsstatus

Offizieller Verteilungsort: [adb-on Downloads und Releases](https://github.com/cj-tinygem/adb-on/releases)

|Plattform|Bereitgestellte Form|Aktueller Status|
|---|---|---|
|Windows x64|`adb-on.exe`|Verfügbar. Mit einem echten Telefon über USB und WLAN geprüft. Das Koppeln per Code wartet noch auf die Prüfung mit echtem Telefon.|
|macOS Apple Silicon|`adb-on.app`|Noch nicht veröffentlicht. Build-Pfad vorbereitet, nicht auf einem echten Mac geprüft.|
|macOS Intel|`adb-on.app`|Noch nicht veröffentlicht. Build-Pfad vorbereitet, nicht auf einem echten Mac geprüft.|

- Es gibt kein Installationsprogramm. Entpacken Sie das Archiv, legen Sie die einzelne Datei `adb-on.exe` in einen beliebigen Ordner und starten Sie sie. Die Lizenzdateien im Zip-Archiv werden für die Ausführung der App nicht benötigt.
- Die Windows-App benötigt weder WebView2 noch Node.js noch Java.
- Die App-Sprache folgt zunächst der Sprache des PCs. In den Werkzeugeinstellungen können Sie zwischen English, 简体中文, 日本語, Español, 한국어, Deutsch und Français wechseln.
- Wenn ADB bereits vorhanden ist, wird es wiederverwendet. Andernfalls bereitet die App nach Zustimmung zu den Nutzungsbedingungen das offizielle Google-Werkzeug vor. Für die erste Vorbereitung ist eine Internetverbindung erforderlich.
- Der aktuelle Entwicklungs-Build ist keine Verteilung mit abgeschlossener offizieller Codesignatur und Beglaubigung. OS-Warnungen werden nicht automatisch aufgehoben, und Sicherheitseinstellungen werden nicht geändert.
- Die angestrebte Mindestversion für macOS ist 12. Der tatsächliche Kompatibilitätsbereich wird nach nativer Prüfung festgelegt.
- Native Unterstützung für Linux und Windows ARM wird derzeit nicht angeboten.

Zu jedem Release werden die **SHA-256-Prüfsumme** der ausführbaren Datei und der Prüfungsumfang bereitgestellt. Lesen Sie die Release-Beschreibung, um Entwicklungs-Testversionen nicht mit offiziell unterstützten Versionen zu verwechseln.

<br>

## Erste Verbindung über USB

### 1. adb-on öffnen

Wenn ein Hinweis erscheint, dass ADB fehlt, klicken Sie auf **Erste Verbindung vorbereiten**. Nach Prüfung und Zustimmung zu den Google-SDK-Nutzungsbedingungen lädt die App die offiziellen Dateien herunter und bereitet sie vor.

Wenn Android Studio bereits installiert ist, wird dieser Schritt in der Regel übersprungen. Wenn Sie das Werkzeug an einem anderen Ort installiert haben, können Sie unter **Werkzeugeinstellungen → Vorhandene ADB-Datei auswählen** die Datei `adb.exe` oder `adb` auswählen.

### 2. Entwickleroptionen am Telefon öffnen

|Telefon|Menüpfad|
|---|---|
|Galaxy|Einstellungen → Telefoninfo → Softwareinformationen → **Buildnummer** 7-mal antippen|
|Pixel und andere|Einstellungen → Über das Telefon → **Build-Nummer** 7-mal antippen|

Wenn ein Sperrkennwort verlangt wird, geben Sie es **direkt am Telefon** ein. Die Menünamen können je nach Modell und Android-Version abweichen.

### 3. USB-Debugging aktivieren

Aktivieren Sie in den Telefoneinstellungen **Entwickleroptionen → USB-Debugging**. Aus Sicherheitsgründen von Android muss dieser Schritt direkt am Telefon erfolgen.

### 4. Mit einem Datenkabel verbinden und zulassen

Entsperren Sie das Telefon und tippen Sie auf **USB-Debugging zulassen? → Zulassen**. Wenn es Ihr eigener PC ist, erleichtert die Option „Immer zulassen“ die nächste Verbindung.

Sobald adb-on zu **Verbunden** wechselt, wählen Sie das Telefon in den Entwicklungswerkzeugen dieses PCs aus.

> Reine Ladekabel können nicht für eine ADB-Verbindung verwendet werden. Ein Telefon, das die Verbindung nicht zugelassen hat, wird vom PC aus nicht zwangsweise freigegeben.

<br>

## Verbindung ohne Kabel

Verwendet das drahtlose Debugging, das ab Android 11 unterstützt wird.

### 1. Mit demselben WLAN verbinden

PC und Telefon müssen sich in einem Netzwerk befinden, in dem sie miteinander kommunizieren können. Firmen-, Schul- und Gast-WLANs können die Kommunikation zwischen Geräten blockieren.

### 2. Kopplungsbildschirm am Telefon öffnen

Wählen Sie **Einstellungen → Entwickleroptionen → Debugging über WLAN → Gerät über Kopplungscode koppeln**. Lassen Sie diesen Bildschirm geöffnet.

### 3. In adb-on auf Drahtlose Verbindung klicken

Klicken Sie auf die gefundene Kopplungsadresse, geben Sie die am Telefon angezeigte **6-stellige Zahl** ein und klicken Sie auf **Koppeln**. Wird nur eine Adresse gefunden, wird sie automatisch eingetragen. Wenn mehrere Telefone angezeigt werden, vergleichen Sie mit der Adresse auf dem Bildschirm Ihres Telefons.

Die automatische Erkennung läuft eine Weile weiter. Auch wenn nichts gefunden wird, können Sie die am Telefon angezeigte **IP-Adresse:Port** direkt eingeben.

### 4. Verbindungsstatus prüfen

Die Kopplung ist der Vorgang, mit dem der PC als vertrauenswürdig registriert wird; sie ist nicht dasselbe wie eine abgeschlossene Verbindung.

Wenn keine Verbindung zustande kommt, gehen Sie am Telefon einen Bildschirm zurück, geben Sie **IP-Adresse und Port vom Startbildschirm von Debugging über WLAN** in das Verbindungsfeld von adb-on ein und klicken Sie auf **Verbinden**.

|Adresse|Wo befindet sie sich?|Wo wird sie eingetragen?|
|---|---|---|
|Kopplungsadresse|Popup „Gerät über Kopplungscode koppeln“|① Feld Kopplungsadresse|
|Verbindungsadresse|Startbildschirm von Debugging über WLAN|② Feld Verbindungsadresse|

**Die beiden Ports sind unterschiedlich.** Zum Beispiel kann die Kopplung `192.168.1.5:37123` und die Verbindung `192.168.1.5:39511` lauten. Kopieren Sie nicht das Beispiel, sondern verwenden Sie die Werte Ihres Telefons.

<br>

## Beim nächsten Verbinden

- **USB:** Bei einem zuvor als vertrauenswürdig eingestuften PC stecken Sie das Kabel ein und prüfen den Status. Wenn erneut eine Bestätigung nötig ist, lassen Sie sie am Telefon zu.
- **Drahtlos:** Wenn im selben Netzwerk das drahtlose Debugging aktiviert ist und ADB das Gerät findet, wird die Verbindung wiederhergestellt. Wenn die automatische Verbindung fehlschlägt, verwenden Sie die **aktuelle Verbindungsadresse** des Telefons.
- **Nach Neustart oder WLAN-Wechsel:** Das drahtlose Debugging kann deaktiviert sein oder IP und Port können sich geändert haben. adb-on speichert keine veralteten Adressen und versucht sie nicht fortlaufend.
- **App beenden:** Beim Schließen des Fensters wird adb-on beendet. Der ADB-Server und die Verbindungen, die andere Entwicklungswerkzeuge verwenden, bleiben erhalten. Es gibt keinen versteckten residenten Dienst von adb-on.
- **Drahtlose Verbindung trennen:** Klicken Sie auf die Trennen-Schaltfläche des jeweiligen Geräts. Da ADB die Verbindung automatisch wiederherstellen kann, deaktivieren Sie das drahtlose Debugging am Telefon, wenn Sie die Verbindung sicher beenden möchten.

<br>

## Schnelle Hilfe bei Problemen

|Beobachtete Situation|Erster Schritt|
|---|---|
|Warten auf Bestätigung am Telefon|Telefon entsperren und Popup zum Zulassen von USB-Debugging prüfen|
|USB eingesteckt, aber Telefon nicht sichtbar|Nacheinander Datenkabel, anderen Anschluss und USB-Debugging prüfen|
|Unter Windows weiterhin nicht sichtbar|[Anleitung zu USB-Treibern der Hersteller](https://developer.android.com/studio/run/oem-usb) prüfen|
|Drahtloses Gerät wird nicht gefunden|Kopplungs-Popup geöffnet lassen oder Adresse direkt eingeben|
|Kopplungscode wird abgelehnt|Am Telefon ein neues Kopplungs-Popup öffnen und neuen Code und neue Adresse eingeben|
|Gekoppelt, aber keine Verbindung|Mit der **Verbindungsadresse** vom Startbildschirm von Debugging über WLAN verbinden|
|Im Firmen-WLAN funktioniert es nicht|Auch im selben Netzwerk kann die Kommunikation blockiert sein. USB-Verbindung verwenden|
|Hinweis auf eine andere ADB-Server-Version|Dieselbe ADB-Datei auswählen wie das verwendete Entwicklungswerkzeug|
|Entwicklungswerkzeug findet das Telefon nicht|Prüfen, ob der Standard-ADB-Server derselben PC-Umgebung verwendet wird. WSL ist eine separate Umgebung|
|ADB-Download fehlgeschlagen|Nach Herstellen der Internetverbindung erneut vorbereiten oder vorhandene ADB-Datei auswählen|

Unter **USB-Verbindungsanleitung** in der App finden Sie dieselben Hinweise Schritt für Schritt. Aus einer leeren Geräteliste allein wird nicht abgeleitet, ob Treiber, Kabel oder Telefoneinstellungen die Ursache sind.

<br>

## Was wird automatisch erledigt?

|Am PC erledigt|Direkt am Telefon zu erledigen|
|---|---|
|Vorhandenes ADB suchen und Verbindungsstatus prüfen|Entwickleroptionen sowie USB- oder drahtloses Debugging aktivieren|
|Nach Zustimmung offizielles ADB herunterladen und Dateien prüfen|Erste Vertrauensbestätigung für den PC|
|Drahtlose Adresse erkennen, Kopplungsbefehl und Verbindungsanfrage senden|Kopplungsbildschirm öffnen und 6-stelligen Code prüfen|
|Passende nächste Schritte je nach Status anzeigen|Berechtigungsprobleme am Telefon lösen, etwa durch Unternehmensrichtlinien|

Bildschirmspiegelung, Dateiverwaltung, App-Build, Rooting und automatische Treiberinstallation werden nicht angeboten.

<br>

## Maßstab für klein und schnell

Leistung wird nicht allein mit einer kleinen ausführbaren Datei behauptet. Vor der Verteilung werden die folgenden Punkte gemessen.

- Zeit vom Start bis zur Anzeige des Fensters.
- CPU und Speicher im Leerlauf. Ein vorhandener gemeinsam genutzter ADB-Server wird getrennt ausgewiesen.
- Größe der ausführbaren Datei und des ersten Downloads.
- Statusaktualisierung beim Verbinden und Trennen des Telefons sowie Reaktion der Oberfläche während der Bedienung.

Die GUI besteht aus **Rust + Slint** und startet keine System-WebView. Statusänderungen nutzen die Benachrichtigungen von ADB, und die Abfrage zur drahtlosen Erkennung läuft nur für begrenzte Zeit, wenn der Benutzer die drahtlose Verbindung öffnet.

Messwerte des Windows-Entwicklungs-Builds (ohne verbundenes Telefon, 2026-09-11):

|Punkt|Ergebnis|
|---|---|
|Ausführbare Datei|ca. 12.0MiB|
|Speicher im Leerlauf|ca. 126MiB Arbeitssatz, ca. 59MiB privat (GPU-Zeichnen; der Großteil des Arbeitssatzes ist Grafiktreiber-Speicher)|
|Fensteranzeige bei drei Neustarts|0.47~0.57 Sekunden|
|Gemeinsam genutzter ADB-Server|separat ca. 4.7MiB, vorhandener Prozess beibehalten|

Bei jedem Lauf wurden in 30 Sekunden Leerlauf 0.13~0.36 Sekunden CPU-Zeit verbraucht (ca. 0.4~1.2% eines Kerns). Seit diesem Build wird das Fenster mit der GPU gezeichnet; das Software-Zeichnen aus den FAQ benötigt ca. 30MiB, scrollt aber weniger flüssig. Der erste Start des vorherigen ersten Builds dauerte 1.43 Sekunden; ein Kaltstart oder die Leistung anderer PCs wird nicht garantiert. Die Last während der Geräteverbindung und die Leistung unter macOS wurden noch nicht geprüft.

<br>

## Was bleibt auf meinem PC?

- Die Einstellung mit dem ADB-Speicherort (`settings.json`) und, falls Sie die Vorbereitung in der App gewählt haben, eine Kopie des offiziellen ADB. Beides liegt ausschließlich im folgenden Ordner.
  - Windows: `%LOCALAPPDATA%\tinygem\adb-on\data\`
  - macOS: `~/Library/Application Support/ai.tinygem.adb-on/`
- Die von ADB selbst verwalteten PC-Authentifizierungsschlüssel und der Server. Vorhandene Schlüssel werden von adb-on nicht gelöscht oder ersetzt.

Kopplungscodes werden nicht in Einstellungen oder Nutzungsprotokollen gespeichert. Es gibt keinen Remote-Speicherserver. Bei der automatischen Vorbereitung wird eine Verbindung zum Google-Downloadserver hergestellt, und Hinweislinks werden im Standardbrowser geöffnet.

Um die App nicht mehr zu verwenden, wählen Sie **Werkzeugeinstellungen → adb-on-Daten löschen**, schließen Sie dann das Fenster und löschen Sie die ausführbare Datei. Wenn Sie die ausführbare Datei bereits gelöscht haben, löschen Sie den oben genannten Ordner selbst. Läuft das hier vorbereitete ADB als Server, wird es beim Löschen beendet; das ADB anderer Werkzeuge und die ADB-Authentifizierungsschlüssel bleiben unberührt. Das Löschen der ausführbaren Datei allein entfernt diesen Ordner nicht.

<br>

## Häufig gestellte Fragen

### Muss ich es installieren?

Nein. Legen Sie die einzelne Datei `adb-on.exe` in einen beliebigen Ordner und starten Sie sie. Die Einstellungen und das vorbereitete ADB werden im oben genannten Benutzerdatenordner gespeichert und können über die Werkzeugeinstellungen gelöscht werden.

### Ist es Open Source?

Ja. Der Quellcode ist unter der MIT-Lizenz veröffentlicht. Komponenten von Drittanbietern unterliegen ihren jeweiligen Lizenzen.

### Muss ich ADB kennen?

Für die grundlegende Verbindung sind keine Befehle nötig. Die am Telefon erforderlichen Bestätigungen und Einstellungen werden auf dem Bildschirm angezeigt.

### Kann ich Apps ohne Android Studio entwickeln?

adb-on ist ein Verbindungswerkzeug. Es ersetzt nicht das SDK und die Build-Werkzeuge, die zum Erstellen von Apps nötig sind.

### Funktioniert die Verbindung unter Windows auch in WSL?

Sie wird nicht automatisch weitergegeben. Windows und WSL können separate Entwicklungsumgebungen sein.

### Werden unter macOS sowohl Intel als auch Apple Silicon unterstützt?

Build-Pfade für beide Architekturen sind vorbereitet, aber es ist noch kein macOS-Build veröffentlicht und keiner auf einem echten Mac geprüft. Ein veröffentlichter Build wird keine Apple-Developer-ID-Signatur tragen, daher ist beim ersten Start einmalig Rechtsklick → Öffnen nötig.

### Wird eine separate App auf dem Telefon installiert?

Für die Verbindung mit adb-on wird keine separate App auf dem Telefon installiert. Es wird die Debugging-Funktion von Android verwendet.

<br>

## Haftungsausschluss

adb-on ist Open-Source-Software, die unter der MIT-Lizenz veröffentlicht wird. Der Lizenztext in [LICENSE.txt](LICENSE.txt) ist die verbindliche Grundlage; die folgenden Punkte geben ihn in einfacher Sprache wieder.

- **Keine Gewährleistung.** Die Software wird „wie besehen“ bereitgestellt, ohne jegliche ausdrückliche oder stillschweigende Gewährleistung. Es gibt keine Garantie, dass sie für einen bestimmten Zweck geeignet ist oder fehlerfrei funktioniert.
- **Keine Haftung.** Die Entwickler und Mitwirkenden haften nicht für Schäden, die aus der Nutzung oder der Unmöglichkeit der Nutzung dieser Software entstehen. Dies umfasst Datenverlust, Fehlfunktionen von Geräten, fehlgeschlagene Verbindungen, Betriebsunterbrechungen und alle sonstigen direkten oder indirekten Schäden. Ob und wie Sie die Software verwenden, liegt vollständig in Ihrer eigenen Entscheidung und Verantwortung.
- **Sie tragen die Risiken der Entwickleroptionen.** Entwickleroptionen sowie USB- und drahtloses Debugging sind Funktionen von Android. Solange sie aktiviert sind, kann ein verbundener PC das Telefon steuern. Aktivieren Sie sie nicht auf PCs oder in Netzwerken, denen Sie nicht vertrauen, und deaktivieren Sie sie, wenn Sie fertig sind. adb-on verwaltet diese Einstellungen nicht für Sie.
- **Werkzeuge von Drittanbietern unterliegen ihren eigenen Bedingungen.** ADB ist Teil der Android SDK Platform-Tools von Google und wird unter den Bedingungen von Google bereitgestellt. adb-on akzeptiert diese Bedingungen nicht in Ihrem Namen. Weitere Komponenten unterliegen den in den [Hinweisen zu Drittanbietern](THIRD-PARTY-NOTICES.md) aufgeführten Lizenzen.
- **Keine Verbindung zu Google.** adb-on steht in keiner Verbindung zu Google oder Android und wird von diesen weder gesponsert noch unterstützt. Android ist eine Marke von Google LLC.
- **Keine Zusage zur Unterstützung.** Funktionen können sich ohne Ankündigung ändern, und die Verteilung kann eingestellt werden. Updates, Fehlerbehebung und fortdauernde Kompatibilität werden nicht zugesichert.
- **Gilt im gesetzlich zulässigen Umfang.** Diese Einschränkungen gelten nicht für Haftung, deren Ausschluss nach geltendem Recht nicht zulässig ist.

### Das Fenster sieht falsch aus oder das Scrollen ruckelt über Remotedesktop

adb-on zeichnet standardmäßig mit der GPU und wechselt von selbst auf Software-Zeichnen, wenn der Grafiktreiber kein OpenGL bereitstellt. Sieht das Fenster trotzdem falsch aus oder fühlt es sich in einer Remotedesktop- oder VM-Sitzung langsam an, starten Sie es mit erzwungenem Software-Zeichnen:

```
cmd /c "set SLINT_BACKEND=winit-software && adb-on.exe"
```

Führen Sie dies im Ordner mit adb-on.exe aus. Es wird nichts auf die Festplatte geschrieben. Unter macOS setzen Sie dieselbe Umgebungsvariable im Terminal, bevor Sie die App starten.

<br>

## Problemmeldung und Nutzungsbedingungen

Teilen Sie uns unter [Problemmeldung](https://github.com/cj-tinygem/adb-on/issues) Folgendes mit.

- Betriebssystem des PCs, Telefonmodell und Android-Version.
- Ob USB oder drahtlos verwendet wurde.
- In welchem Schritt welcher Hinweis angezeigt wird.

**Senden Sie keine Kopplungscodes, PC-Authentifizierungsschlüssel oder personenbezogenen Daten.** Bitte schwärzen Sie auch personenbezogene Daten in Screenshots.

[Lizenz](LICENSE.txt) · [Hinweise zu Drittanbietern](THIRD-PARTY-NOTICES.md) · Erstellt von cj bei tinygem

<br>

## Offizielle Referenzen

- [Offizielle Android-ADB-Dokumentation](https://developer.android.com/tools/adb)
- [Apps auf einem echten Gerät ausführen](https://developer.android.com/studio/run/device)
- [Offizielle Platform-Tools](https://developer.android.com/tools/releases/platform-tools)
- [USB-Treiber der Hersteller für Windows](https://developer.android.com/studio/run/oem-usb)
- [Slint](https://slint.dev)

<p align="center"><a href="https://slint.dev"><img src="https://github.com/slint-ui/slint/raw/master/logo/MadeWithSlint-logo-light.svg" width="160" alt="Made with Slint"></a></p>

Die Dokumentstruktur orientiert sich fortlaufend am Inhaltsverzeichnis, den visuellen Erläuterungen und der Navigierbarkeit von [System Design Primer](https://github.com/donnemartin/system-design-primer). Passend zum Zweck von adb-on werden ein kurzer Einstiegspfad und eine ausführliche Fehlerbehebung gemeinsam bereitgestellt.
