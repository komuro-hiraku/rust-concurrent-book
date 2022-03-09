struct Resource<const NRES:usize, const NTH: usize> {

    available: [usize; NRES],           // 利用可能なリソース
    allocation: [[usize; NRES]; NTH],   // スレッド i が確保中のリソース
    max: [[usize; NRES]; NTH],          // スレッド i が必要とするリソースの最大値
}

impl<const NRES:usize, const NTH:usize> Resource<NRES, NTH> {

    // 構造体を初期化
    fn new(available: [usize; NRES], max: [[usize; NRES]; NTH]) -> Self {
        Resource {
            available,
            allocation: [[0; NRES]; NTH],
            max
        }
    }

    fn is_safe(&self) -> bool {
        let mut finish = [false; NTH];
        let mut work = self.available.clone();  // 利用可能なリソースのシミュレートに利用するため Clone

        loop {

            let mut found = false;
            let mut num_true = 0;

            for (i, alc) in self.allocation.iter().enumerate() {
                if finish[i] {
                    num_true += 1;
                    continue;
                }

                // need[j] = self.max[i][j] - self.allocation[i][j] を計算
                let need = self.max[i].iter().zip(alc).map(| (m, a) | m - a);

                // need 全部と available を組み合わせて work >= need を全部満たしてるか確認
                let is_avail = work.iter().zip(need).all(|(w, n)| *w >= n);

                if is_avail {
                    // スレッド i がリソース確保可能
                    found = true;
                    finish[i] = true;

                    for (w, a) in work.iter_mut().zip(alc) {
                        *w += *a    // スレッドiの現在確保しているリソースを返却    // この返却値どこに返る？
                    }
                    break;
                }
            }

            if num_true == NTH {
                return true;
            }

            if !found {
                // リソースを確保できないスレッドがある
                break;  // loop からの脱出  // ? ここで return false しない理由は？
            }
        }

        false
    }
    
}
