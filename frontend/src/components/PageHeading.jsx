function getText(number) {
    if (number == 1) return "položka";
    if (number > 1 && number < 5) return "položky";
    return "položiek";
}

function PageHeading({title, number}) {
    return (
        <div className={"self-start"}>
            <h1>{title}</h1>
            { number != undefined && <p>{number} {getText(number)}</p> }
        </div>
    )
}

export default PageHeading