function Footer() {
    return (
        <footer className={"flex justify-center items-center gap-10 p-8 bg-zinc-950 portrait:flex-col portrait:items-start"}>
            <div className={"flex flex-col justify-center items-end gap-1 portrait:items-start"}>
                <a
                    className={"!text-3xl !font-semibold portrait:!text-2xl"}
                    href="https://github.com/cyprich/filamenty-rs"
                    target={"_blank"}
                >Filamenty</a>
                <p>© 2025 - 2026</p>
            </div>
            <div className={"flex flex-col portrait:gap-1.5"}>
                <h3 className={"!font-medium portrait:!text-lg"}>Peter Cyprich</h3>
                <a href={"mailto:cypooriginal@gmail.com"} target={"_blank"}>cypooriginal@gmail.com</a>
                <a href={"mailto:cyprich6@stud.uniza.sk"} target={"_blank"}>cyprich6@stud.uniza.sk</a>
            </div>
        </footer>
    )
}

export default Footer;