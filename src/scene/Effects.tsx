import { EffectComposer, Bloom, ToneMapping } from '@react-three/postprocessing';
import { KernelSize, Resolution } from 'postprocessing';

export function Effects() {
  return (
    <EffectComposer>
      <Bloom
        intensity={1.25}
        luminanceThreshold={1}   // toneMapped=false로 1 초과된 발광체만 bloom
        luminanceSmoothing={0.03}
        mipmapBlur
        kernelSize={KernelSize.LARGE}
        resolutionX={Resolution.AUTO_SIZE}
        resolutionY={Resolution.AUTO_SIZE}
      />
      <ToneMapping />
    </EffectComposer>
  );
}
