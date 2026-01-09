import {useNavigate} from "react-router-dom";

function PlusButton({redirect}) {
    const navigate = useNavigate();

    return (
        <div
            className={
                "clickable-small custom-border border px-32 py-8 mx-auto w-max" +
                "portrait:py-6 portrait:w-full portrait:-mt-6 portrait:text-center"
            }
            onClick={() => navigate(redirect)}>
            <p className={"text-4xl"}>+</p>
        </div>
    )
}

export default PlusButton;