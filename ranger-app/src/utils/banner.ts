import nunjucks from 'nunjucks';
import {type Banner} from 'src/models/exercise';

export const parseBannerForParticipant = (
  banner: Banner,
  exerciseName: string,
  deploymentName: string,
  username: string,
) => ({
  ...banner,
  content: nunjucks.renderString(banner.content, {
    exerciseName,
    deploymentName,
    username,
  }),
});
