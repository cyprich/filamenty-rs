function getText(number) {
    if (number == 1) return "položka";
    if (number > 1 && number < 5) return "položky";
    return "položiek";
}

function PageTitle({title, number}) {
    return (
        <div className={"self-start no-print"}>
            <h1>{title}</h1>
            { number != undefined && <p>{number} {getText(number)}</p> }
        </div>
    )
}

export default PageTitle