import { useForm, SubmitHandler } from 'react-hook-form';
import axios from 'axios';
import { Button, Intent, Label } from '@blueprintjs/core';
import { AppToaster } from "../components/toaster";

const styles = {
    container: {
        width: "50%",
    },
    input: {
        width: "100%",
        margin: "10px",
    },
    textArea: {
        width: "100%",
        margin: "10px",
        height: "200px",
    },
};

type Exercise = {
    name: string,
    scenario: string,
};

const ExerciseForm = () => {

    const { register, handleSubmit, formState: { errors } } = useForm<Exercise>();

    const onSubmit: SubmitHandler<Exercise> = async exercise => {
        await axios.post('api/v1/exercise', JSON.stringify(exercise), {
            headers: {
                'Content-Type': 'application/json'
            },
        }).then((response) => {
            AppToaster.show({
                icon: "tick",
                intent: Intent.SUCCESS,
                message: "Added exercise with uuid: " + response.data.uuid
            });
        }).catch((error) => {
            AppToaster.show({
                icon: "warning-sign",
                intent: Intent.DANGER,
                message: "Can't add exercise: " + error.response.data
            });
        });
    };

    return (
        <div style={styles.container} >
            <h3>Add new exercise</h3>
            <form className="ExerciseForm" onSubmit={handleSubmit(onSubmit)} >
                <Label>
                    Exercise name
                    <input
                        placeholder="exercise-1"
                        {...register("name")}
                        style={styles.input}
                    />
                </Label>
                <Label>
                    Scenario yaml
                    <textarea
                        placeholder="scenario: ..."
                        {...register('scenario')}
                        style={styles.textArea}
                    />
                </Label>
                <Button type="submit" > Submit </Button>
            </form>
        </div>
    );
}

export default ExerciseForm;