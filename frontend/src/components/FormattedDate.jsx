function FormattedDate({date}) {
    const formatted = new Date(date).toLocaleString()
    return (<span>{formatted}</span>)
}

export default FormattedDate