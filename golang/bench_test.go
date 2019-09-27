package bench

import (
	"crypto/rand"
	"crypto/sha256"
	"hash"
	"testing"
)

func benchmarkRun(h hash.Hash, i int, b *testing.B) {
	bs := make([]byte, i)
	_, err := rand.Read(bs)
	if err != nil {
		b.Fatal(err)
	}
	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		h.Write(bs)
		h.Sum(nil)
	}

}

func BenchmarkSha256_32(b *testing.B) {
	benchmarkRun(sha256.New(), 32, b)
}
