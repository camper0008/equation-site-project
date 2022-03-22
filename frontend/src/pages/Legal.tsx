import "../assets/legal_page.scss";

const Legal = () => {
    const WEBSITE_NAME = "Formelsamling.dk";
    const WEBSITE_URL = "https://formelsamling.dk";
    return (
        <div class="legal-container">
            <h1>Privatlivspolitik</h1>
            <p>{WEBSITE_NAME} gemmer følgende data:</p>
            <ul>
                <li>
                    Ved registrering:{" "}
                    <ul>
                        <li>
                            Brugernavn og Hashed<sup>1</sup> password
                        </li>
                        <li>
                            Tilfældigt genereret identifikationsnummer
                            <sup>2</sup>
                        </li>
                        <li>
                            Bruger adgangsniveau<sup>3</sup>
                        </li>
                        <li>Dato, som konto bliver skabt</li>
                    </ul>
                </li>
                <li>
                    Ved login:{" "}
                    <ul>
                        <li>
                            En cookie<sup>4</sup>, der dikterer din nuværende
                            session. Den opbevares sammen med reference til dit
                            bruger identifikationsnummer.
                        </li>
                    </ul>
                </li>
                <li>
                    Ved skabning af formel, gennem siden:{" "}
                    <ul>
                        <li>Dato, formlen er skabt</li>
                        <li>Bruger identifikationsnummer</li>
                        <li>Formel indhold, herunder titel og tekst</li>
                        <li>
                            Et tilfældigt genereret identifikationsnummer
                            tilhørende formlen
                        </li>
                    </ul>
                </li>
            </ul>
            <h2>Kilde</h2>
            Dette projekt er open-source, og kan derfor findes på{" "}
            <a
                href="https://www.github.com/camper0008/equation-site-project"
                target="_blank"
            >
                GitHub
            </a>
            .<h2>Fodnoter</h2>
            <ol>
                <li>
                    Et "hashed" password, er per definition etvejs, og det er
                    derfor umuligt at få réelt password tilbage. Mere
                    information kan findes her:{" "}
                    <a
                        href="https://en.wikipedia.org/wiki/Hash_function"
                        target="_blank"
                    >
                        Hash function - Wikipedia
                    </a>
                </li>
                <li>Dette bruges for standard funktionalitet</li>
                <li>
                    Adgangsniveau dikterer, om du har rettigheder til at f.eks.
                    skabe formler, på siden.
                </li>
                <li>
                    En cookie er en lille fil, der holder på noget data. Din
                    SESSION_ID cookie holder på et tilfældigt generet
                    identifikationsnummer, for din session.
                </li>
            </ol>
        </div>
    );
};

export default Legal;
