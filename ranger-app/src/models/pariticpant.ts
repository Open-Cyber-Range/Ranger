type NewParticipant = {
  userId: string;
  selector: string;
};

type Participant = {
  id: string;
  deployment_id: string;
  user_id: string;
  selector: string;
};

export type {NewParticipant, Participant};
