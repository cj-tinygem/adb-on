<p align="center"><em><a href="README.md">English</a> ∙ <a href="README-zh-Hans.md">简体中文</a> ∙ <a href="README-ja.md">日本語</a> ∙ <a href="README-es.md">Español</a> ∙ <a href="README-ko.md">한국어</a> ∙ <a href="README-de.md">Deutsch</a> ∙ <a href="README-fr.md">Français</a></em></p>

<br>

<div align="center">
  <img src="docs/assets/icon.svg" width="88" alt="Icône adb-on représentant la connexion d'un téléphone">
  <h1>adb-on</h1>
  <p><strong>Connecter votre téléphone, depuis une seule fenêtre.</strong></p>
  <p>Un petit outil de bureau simple pour connecter un téléphone Android sans ouvrir de terminal.</p>
  <p>Fonctionne sous Windows. Une version macOS est en préparation.</p>
</div>

> **Version Windows disponible.** Téléchargez `adb-on-windows-x64.zip` depuis les Releases, décompressez-le et ouvrez `adb-on.exe`. La connexion avec un téléphone réel a été validée en USB et en Wi-Fi. Les builds macOS ne sont pas encore publiés.

<p align="center"><img src="docs/assets/windows.png" width="400" alt="Écran réel d'attente de connexion d'adb-on exécuté sous Windows"></p>

**L'objectif est simple.** Lancez l'application et connectez votre téléphone. adb-on se charge de ce qui doit être fait sur le PC et vous indique, au moment voulu, ce que vous devez confirmer sur le téléphone.

<br>

## Sommaire

- [Pourquoi adb-on ?](#pourquoi-adb-on-)
- [Téléchargement et état de la prise en charge](#téléchargement-et-état-de-la-prise-en-charge)
- [Première connexion par USB](#première-connexion-par-usb)
- [Connexion sans câble](#connexion-sans-câble)
- [Lors de la prochaine reconnexion](#lors-de-la-prochaine-reconnexion)
- [En cas de blocage](#en-cas-de-blocage)
- [Que fait adb-on automatiquement ?](#que-fait-adb-on-automatiquement-)
- [Critères de légèreté et de rapidité](#critères-de-légèreté-et-de-rapidité)
- [Que reste-t-il sur mon PC ?](#que-reste-t-il-sur-mon-pc-)
- [Questions fréquentes](#questions-fréquentes)
- [Clause de non-responsabilité](#clause-de-non-responsabilité)
- [Signalement de problèmes et conditions d'utilisation](#signalement-de-problèmes-et-conditions-dutilisation)
- [Références officielles](#références-officielles)

<br>

## Pourquoi adb-on ?

Vous avez créé une application Android, mais vous êtes bloqué au moment de la connecter à votre téléphone ?

Chercher l'emplacement d'installation d'`adb`, copier des commandes et ressaisir un port qui a changé ne fait pas partie du cœur du développement d'une application. adb-on regroupe dans une petite fenêtre uniquement ce qui est nécessaire à la connexion.

- **USB vérifié automatiquement :** distingue les états connecté, en attente d'autorisation sur le téléphone et sans réponse.
- **Sans fil, étape par étape :** sélectionnez l'adresse découverte et saisissez le code à 6 chiffres affiché sur le téléphone. Si la découverte automatique échoue, vous pouvez saisir l'adresse manuellement.
- **La première fois, une étape à la fois :** vous guide des options pour les développeurs jusqu'au bouton d'autorisation sur le téléphone.
- **Compatible avec vos outils existants :** utilise le serveur ADB par défaut. Il n'arrête pas systématiquement un serveur existant et ne supprime pas les clés d'authentification.
- **Uniquement l'essentiel :** se concentre sur la connexion, sans serveur cloud, ni mise en miroir, ni gestionnaire de fichiers.

<br>

## Téléchargement et état de la prise en charge

Emplacement de distribution officiel : [Téléchargements et versions d'adb-on](https://github.com/cj-tinygem/adb-on/releases)

|Plateforme|Format fourni|État actuel|
|---|---|---|
|Windows x64|`adb-on.exe`|Disponible. Validé avec un téléphone réel en USB et en Wi-Fi. L'association par code attend encore une validation sur téléphone réel.|
|macOS Apple Silicon|`adb-on.app`|Pas encore publié. Chemin de compilation prêt, non validé sur un Mac réel.|
|macOS Intel|`adb-on.app`|Pas encore publié. Chemin de compilation prêt, non validé sur un Mac réel.|

- Il n'y a pas d'installateur. Décompressez l'archive, placez le fichier unique `adb-on.exe` dans le dossier de votre choix et exécutez-le. Les fichiers de licence contenus dans l'archive zip ne sont pas nécessaires pour exécuter l'application.
- L'application Windows ne nécessite ni WebView2, ni Node.js, ni Java.
- La langue de l'application suit d'abord celle du PC. Dans les paramètres des outils, vous pouvez choisir entre English, 简体中文, 日本語, Español, 한국어, Deutsch et Français.
- Si ADB est déjà présent, il est réutilisé. Sinon, l'application prépare l'outil officiel de Google après acceptation des conditions. Une connexion Internet est nécessaire pour la préparation initiale.
- La version de développement actuelle n'est pas une distribution signée et notarisée par une autorité reconnue. Elle ne contourne pas automatiquement les avertissements du système et ne modifie pas les paramètres de sécurité.
- La version minimale visée pour macOS est 12. La plage de compatibilité réelle sera fixée après validation native.
- La prise en charge native de Linux et de Windows ARM n'est pas proposée actuellement.

Chaque version fournit la **somme de contrôle SHA-256** de l'exécutable ainsi que l'étendue de la validation. Consultez la description de la version pour ne pas confondre une version de test de développement avec une version officiellement prise en charge.

<br>

## Première connexion par USB

### 1. Ouvrez adb-on

Si un message indique qu'ADB est absent, cliquez sur **Préparer la première connexion**. Après lecture et acceptation des conditions du SDK Google, l'application télécharge et prépare les fichiers officiels.

Si Android Studio est déjà installé, cette étape est généralement ignorée. Si vous avez installé les outils à un autre emplacement, vous pouvez sélectionner `adb.exe` ou `adb` dans **Paramètres des outils → Sélectionner un fichier ADB existant**.

### 2. Ouvrez les options pour les développeurs sur le téléphone

|Téléphone|Menu à suivre|
|---|---|
|Galaxy|Paramètres → À propos du téléphone → Informations sur le logiciel → appuyer 7 fois sur **Numéro de build**|
|Pixel et autres|Paramètres → À propos du téléphone → appuyer 7 fois sur **Numéro de build**|

Si un mot de passe de verrouillage est demandé, saisissez-le **directement sur le téléphone**. Les noms de menu peuvent varier selon le modèle et la version d'Android.

### 3. Activez le débogage USB

Activez **Options pour les développeurs → Débogage USB** dans les paramètres du téléphone. Pour des raisons de sécurité d'Android, cette opération doit être effectuée directement sur le téléphone.

### 4. Connectez un câble de données et autorisez

Déverrouillez le téléphone et appuyez sur **Autoriser le débogage USB ? → Autoriser**. S'il s'agit de votre propre PC, l'option « Toujours autoriser » facilite les connexions suivantes.

Lorsque adb-on affiche **Connecté**, sélectionnez le téléphone dans l'outil de développement de ce PC.

> Un câble de charge uniquement ne peut pas être utilisé pour une connexion ADB. Un téléphone non autorisé n'est pas approuvé de force depuis le PC.

<br>

## Connexion sans câble

Utilise le débogage sans fil pris en charge à partir d'Android 11.

### 1. Connectez-vous au même Wi-Fi

Le PC et le téléphone doivent être sur un réseau qui leur permet de communiquer entre eux. Les réseaux Wi-Fi d'entreprise, d'école ou invités peuvent bloquer la communication entre appareils.

### 2. Ouvrez l'écran d'association sur le téléphone

Sélectionnez **Paramètres → Options pour les développeurs → Débogage sans fil → Associer l'appareil avec un code d'association**. Laissez cet écran ouvert.

### 3. Cliquez sur Connexion sans fil dans adb-on

Cliquez sur l'adresse d'association découverte, saisissez les **6 chiffres** affichés sur le téléphone, puis cliquez sur **Associer**. Si une seule adresse est découverte, elle est remplie automatiquement. Si plusieurs téléphones apparaissent, comparez avec l'adresse affichée sur l'écran de votre téléphone.

La découverte automatique se poursuit pendant un moment. Si elle n'aboutit pas, vous pouvez saisir directement l'**adresse IP:port** affichée sur le téléphone.

### 4. Vérifiez l'état de la connexion

L'association enregistre le PC comme appareil de confiance ; ce n'est pas la même chose qu'une connexion établie.

Si la connexion échoue, revenez d'un écran en arrière sur le téléphone, saisissez l'**adresse IP et le port de l'écran principal du débogage sans fil** dans le champ de connexion d'adb-on, puis cliquez sur **Connecter**.

|Adresse|Où la trouver ?|Où la saisir ?|
|---|---|---|
|Adresse d'association|Fenêtre « Associer l'appareil avec un code d'association »|① Champ Adresse d'association|
|Adresse de connexion|Écran principal du débogage sans fil|② Champ Adresse de connexion|

**Les deux ports sont différents.** Par exemple, l'association peut utiliser `192.168.1.5:37123` et la connexion `192.168.1.5:39511`. Ne copiez pas l'exemple ; utilisez les valeurs affichées sur votre téléphone.

<br>

## Lors de la prochaine reconnexion

- **USB :** si le PC a déjà été approuvé, branchez le câble et vérifiez l'état. Si une nouvelle autorisation est demandée, acceptez-la sur le téléphone.
- **Sans fil :** sur le même réseau, si le débogage sans fil est activé et qu'ADB détecte l'appareil, la connexion est rétablie. Si la reconnexion automatique échoue, utilisez l'**adresse de connexion actuelle** du téléphone.
- **Après un redémarrage ou un changement de Wi-Fi :** le débogage sans fil peut être désactivé, ou l'IP et le port peuvent changer. adb-on ne conserve pas une ancienne adresse pour la réessayer indéfiniment.
- **Fermeture de l'application :** adb-on se termine lorsque vous fermez la fenêtre. Le serveur ADB et les connexions utilisés par d'autres outils de développement sont conservés. Il n'y a aucun service résident caché d'adb-on.
- **Déconnexion sans fil :** cliquez sur le bouton de déconnexion de l'appareil concerné. ADB pouvant se reconnecter automatiquement, désactivez le débogage sans fil sur le téléphone pour couper la connexion de manière certaine.

<br>

## En cas de blocage

|Situation observée|Première chose à faire|
|---|---|
|En attente d'autorisation sur le téléphone|Déverrouiller le téléphone et vérifier la fenêtre d'autorisation du débogage USB|
|USB branché mais téléphone invisible|Vérifier dans l'ordre : câble de données, autre port, débogage USB|
|Toujours invisible sous Windows|Consulter le [guide des pilotes USB des fabricants](https://developer.android.com/studio/run/oem-usb)|
|Aucun appareil sans fil détecté|Laisser la fenêtre d'association ouverte ou saisir l'adresse manuellement|
|Code d'association refusé|Ouvrir une nouvelle fenêtre d'association sur le téléphone et saisir le nouveau code et la nouvelle adresse|
|Associé mais non connecté|Se connecter avec l'**adresse de connexion** de l'écran principal du débogage sans fil|
|Échec sur le Wi-Fi d'entreprise|La communication peut être bloquée même sur le même réseau. Utiliser la connexion USB|
|Message concernant une autre version du serveur ADB|Sélectionner le même fichier ADB que celui utilisé par votre outil de développement|
|L'outil de développement ne trouve pas le téléphone|Vérifier qu'il utilise le serveur ADB par défaut du même environnement PC. WSL est un environnement distinct|
|Échec du téléchargement d'ADB|Réessayer la préparation après connexion à Internet, ou sélectionner un fichier ADB existant|

Les mêmes indications sont disponibles étape par étape dans l'application, sous **Guide de connexion USB**. Une liste d'appareils vide ne permet pas à elle seule de déterminer si la cause vient du pilote, du câble ou des paramètres du téléphone.

<br>

## Que fait adb-on automatiquement ?

|Pris en charge sur le PC|À faire directement sur le téléphone|
|---|---|
|Recherche d'un ADB existant et vérification de l'état de connexion|Activation des options pour les développeurs et du débogage USB/sans fil|
|Téléchargement de l'ADB officiel après acceptation et vérification des fichiers|Première approbation de confiance du PC|
|Découverte de l'adresse sans fil, commande d'association et demande de connexion|Ouverture de l'écran d'association et vérification du code à 6 chiffres|
|Indication de l'action suivante selon l'état|Résolution des problèmes d'autorisation du téléphone, comme les règles de gestion d'entreprise|

La mise en miroir de l'écran, la gestion de fichiers, la compilation d'applications, le rootage et l'installation automatique de pilotes ne sont pas proposés.

<br>

## Critères de légèreté et de rapidité

La taille réduite de l'exécutable ne suffit pas à revendiquer des performances. Les éléments suivants sont mesurés avant chaque distribution.

- Temps entre le lancement et l'affichage de la fenêtre.
- CPU et mémoire au repos. Le serveur ADB partagé existant est comptabilisé séparément.
- Taille de l'exécutable et du téléchargement initial.
- Mise à jour de l'état lors de la connexion et de la déconnexion du téléphone, et réactivité de l'écran pendant les manipulations.

L'interface est construite en **Rust + Slint** et n'ouvre pas de WebView système. Les changements d'état s'appuient sur les notifications d'ADB, et les requêtes de découverte sans fil ne sont effectuées que pendant une durée limitée, lorsque vous ouvrez la connexion sans fil.

Mesures de la version de développement Windows (sans téléphone connecté, 2026-09-11) :

|Élément|Résultat|
|---|---|
|Exécutable|environ 12.0MiB|
|Mémoire au repos|environ 126MiB de working set, environ 59MiB privés (dessin par GPU ; l'essentiel du working set est de la mémoire du pilote graphique)|
|Affichage de la fenêtre sur trois lancements|0.47~0.57 s|
|Serveur ADB partagé|environ 4.7MiB séparément, processus existant conservé|

À chaque lancement, 30 secondes de repos ont consommé 0.13~0.36 s de temps CPU (environ 0.4~1.2 % d'un cœur). Depuis cette version, la fenêtre est dessinée par le GPU ; le dessin logiciel décrit dans la FAQ utilise environ 30MiB mais le défilement est moins fluide. Le premier lancement de la première version antérieure avait pris 1.43 s ; aucune garantie n'est donnée pour un démarrage à froid ou pour les performances d'un autre PC. La charge pendant la connexion d'un appareil et les performances sous macOS n'ont pas encore été vérifiées.

<br>

## Que reste-t-il sur mon PC ?

- Le paramètre d'emplacement d'ADB (`settings.json`) et, si vous avez choisi la préparation dans l'application, une copie de l'ADB officiel. Les deux se trouvent uniquement dans le dossier ci-dessous.
  - Windows : `%LOCALAPPDATA%\tinygem\adb-on\data\`
  - macOS : `~/Library/Application Support/ai.tinygem.adb-on/`
- Les clés d'authentification du PC et le serveur gérés par ADB lui-même. adb-on ne supprime ni ne remplace les clés existantes.

Les codes d'association ne sont pas enregistrés dans les paramètres ni dans les journaux d'utilisation. Il n'y a pas de serveur de stockage distant. Lors de la préparation automatique, l'application se connecte au serveur de téléchargement de Google, et les liens d'aide s'ouvrent dans le navigateur par défaut.

Pour cesser d'utiliser l'application, choisissez **Paramètres des outils → Supprimer les données d'adb-on**, puis fermez la fenêtre et supprimez l'exécutable. Si vous avez déjà supprimé l'exécutable, supprimez vous-même le dossier ci-dessus. Si l'ADB préparé ici est en cours d'exécution comme serveur, il est arrêté pendant la suppression ; l'ADB des autres outils et les clés d'authentification ADB ne sont pas touchés. La suppression de l'exécutable seule ne supprime pas ce dossier.

<br>

## Questions fréquentes

### Faut-il l'installer ?

Non. Placez le fichier unique `adb-on.exe` dans le dossier de votre choix et exécutez-le. Les paramètres et l'ADB préparé sont enregistrés dans le dossier de données utilisateur indiqué ci-dessus et peuvent être supprimés depuis Paramètres des outils.

### Est-ce un logiciel open source ?

Oui. Le code source est publié sous licence MIT. Les composants tiers sont soumis à leurs licences respectives.

### Faut-il connaître ADB ?

Aucune commande n'est nécessaire pour la connexion de base. Les autorisations et les réglages à effectuer sur le téléphone sont indiqués à l'écran.

### Peut-on développer une application sans Android Studio ?

adb-on est un outil de connexion. Il ne remplace pas le SDK ni les outils de compilation nécessaires pour construire une application.

### Une connexion sous Windows fonctionne-t-elle aussi dans WSL ?

Elle n'est pas transmise automatiquement. Windows et WSL peuvent constituer des environnements de développement distincts.

### Sous macOS, Intel et Apple Silicon sont-ils tous deux pris en charge ?

Les chemins de compilation pour les deux architectures sont prêts, mais aucun build macOS n'est encore publié ni validé sur un Mac réel. Lorsqu'il sera publié, il ne portera pas de signature Apple Developer ID : la première ouverture demandera un clic droit → Ouvrir, une seule fois.

### Faut-il installer une application sur le téléphone ?

Aucune application supplémentaire n'est installée sur le téléphone pour la connexion avec adb-on. La fonction de débogage d'Android est utilisée.

<br>

## Clause de non-responsabilité

adb-on est un logiciel open source publié sous licence MIT. Le texte de la licence dans [LICENSE.txt](LICENSE.txt) constitue les conditions contraignantes ; les points ci-dessous le reformulent en langage courant.

- **Aucune garantie.** Le logiciel est fourni « tel quel », sans garantie d'aucune sorte, expresse ou implicite. Rien ne garantit qu'il convient à un usage particulier ni qu'il fonctionne sans erreur.
- **Aucune responsabilité.** Les développeurs et les contributeurs ne sont pas responsables des dommages découlant de l'utilisation, ou de l'impossibilité d'utilisation, de ce logiciel. Cela comprend la perte de données, le dysfonctionnement d'un appareil, l'échec d'une connexion, l'interruption d'activité et tout autre dommage direct ou indirect. La décision d'utiliser le logiciel et la manière de le faire relèvent entièrement de votre choix et de votre responsabilité.
- **Vous gérez les risques liés aux options pour les développeurs.** Les options pour les développeurs et le débogage USB/sans fil sont des fonctions d'Android. Tant qu'ils sont activés, un PC connecté peut contrôler le téléphone. Ne les activez pas sur des PC ou des réseaux auxquels vous ne faites pas confiance, et désactivez-les une fois terminé. adb-on ne gère pas ces réglages à votre place.
- **Les outils tiers sont soumis à leurs propres conditions.** ADB fait partie des Android SDK Platform-Tools de Google et est fourni selon les conditions de Google. adb-on n'accepte pas ces conditions à votre place. Les autres composants sont soumis aux licences indiquées dans [Notices relatives aux tiers](THIRD-PARTY-NOTICES.md).
- **Aucune affiliation avec Google.** adb-on n'est ni affilié à, ni sponsorisé, ni approuvé par Google ou Android. Android est une marque de Google LLC.
- **Aucun engagement de support.** Les fonctionnalités peuvent changer ou la distribution peut cesser sans préavis. Les mises à jour, la résolution des problèmes et la compatibilité future ne sont pas promises.
- **Dans la mesure permise par la loi.** Ces limitations ne s'appliquent pas aux responsabilités que la loi applicable n'autorise pas à exclure.

### La fenêtre s'affiche mal ou le défilement saccade via le Bureau à distance

adb-on dessine par défaut avec le GPU et passe de lui-même au dessin logiciel lorsque le pilote graphique ne fournit pas OpenGL. Si la fenêtre s'affiche encore mal, ou semble lente dans une session Bureau à distance ou une machine virtuelle, lancez-le en forçant le dessin logiciel :

```
cmd /c "set SLINT_BACKEND=winit-software && adb-on.exe"
```

Exécutez cette commande dans le dossier contenant adb-on.exe. Rien n'est écrit sur le disque. Sous macOS, définissez la même variable d'environnement dans le Terminal avant de lancer l'application.

<br>

## Signalement de problèmes et conditions d'utilisation

Dans [Signalement de problèmes](https://github.com/cj-tinygem/adb-on/issues), indiquez les éléments suivants.

- Le système d'exploitation du PC, le modèle du téléphone et la version d'Android.
- La méthode utilisée : USB ou sans fil.
- L'étape concernée et le message affiché.

**N'envoyez pas de code d'association, de clé d'authentification du PC ni de données personnelles.** Masquez également les informations personnelles dans les captures d'écran.

[Licence](LICENSE.txt) · [Notices tierces](THIRD-PARTY-NOTICES.md) · Créé par cj chez tinygem

<br>

## Références officielles

- [Documentation officielle d'Android ADB](https://developer.android.com/tools/adb)
- [Exécuter des applications sur un appareil réel](https://developer.android.com/studio/run/device)
- [Platform-Tools officiels](https://developer.android.com/tools/releases/platform-tools)
- [Pilotes USB des fabricants pour Windows](https://developer.android.com/studio/run/oem-usb)
- [Slint](https://slint.dev)

<p align="center"><a href="https://slint.dev"><img src="https://github.com/slint-ui/slint/raw/master/logo/MadeWithSlint-logo-light.svg" width="160" alt="Made with Slint"></a></p>

La structure de ce document s'inspire en continu du sommaire, des explications visuelles et de la navigabilité de [System Design Primer](https://github.com/donnemartin/system-design-primer). Conformément à l'objectif d'adb-on, il propose à la fois un parcours de démarrage court et une résolution de problèmes détaillée.
