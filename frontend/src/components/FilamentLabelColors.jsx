function FilamentLabelColors({color}) {
    return (
        <>
            <div
                className={"w-10 h-10 rounded-bl-[90%] absolute top-0 right-0 -z-10"}
                style={{background: `${color}`}}
            />
            <div
                className={"w-10 h-10 rounded-tr-[90%] absolute bottom-0 left-0 -z-10"}
                style={{background: `${color}`}}
            />
        </>
    )
}

export default FilamentLabelColors