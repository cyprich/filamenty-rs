function FilamentCardBar({nettoWeight, originalWeight, colorHex, colorName}) {
    return (
        <div className={"w-full flex flex-col items-center"}>
            <div
                className="h-2.5 rounded-full"
                style={{
                    background: `${colorHex}`,
                    width: `${Math.min(nettoWeight / originalWeight, originalWeight) * 100}%`,
                }}
            />
            <p>{colorName}</p>
        </div>
    )
}

export default FilamentCardBar