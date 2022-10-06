import React from 'react';
import { useForm, SubmitHandler } from 'react-hook-form';
import axios from 'axios';
import { Button, Label } from '@blueprintjs/core';

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

    const { register, handleSubmit } = useForm<Exercise>();
    const onSubmit: SubmitHandler<Exercise> = async exercise => {
        console.log(exercise);

        await axios.post('api/v1/exercise', JSON.stringify(exercise), {
            headers: {
                'Content-Type': 'application/json'
            },
        }).then((response) => {
            console.log(response.data);
        }).catch((error) => {
            console.log(error.response.data);
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
                        {...register("name", { required: true })}
                        style={styles.input}
                    />
                </Label>
                <Label>
                    Scenario yaml
                    <textarea
                        placeholder="scenario: ..."
                        {...register('scenario', { required: true })}
                        style={styles.textArea}
                    />
                </Label>
                <Button type="submit" > Submit </Button>
            </form>
        </div>
    );

}

export default ExerciseForm;